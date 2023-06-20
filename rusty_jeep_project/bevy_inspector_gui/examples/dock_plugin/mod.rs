use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::collections::BTreeMap;
use std::sync::Arc;

/// WidgetInsertClosure Type Aliasing Complex Traits
/// - `Arc`
/// This is a smart pointer that provides shared ownership of a heap-allocated  
/// value. It stands for "Atomic Reference Counting" and ensures that the value  
/// will be deallocated when the last reference to it is dropped. The reference  
/// counting is thread-safe, which means it can be shared between threads  
/// without additional synchronization.
/// - `dyn Fn(&mut egui::Ui)`
/// This is a trait object representing a function pointer. It is a dynamically  
/// dispatched function that takes a mutable reference to an `egui::Ui` object.
/// The `dyn` keyword indicates that the function pointer can refer to any object
/// implementing the `Fn` trait with the given function signature.
/// - + Send + Sync
/// Traits for thread safety so they can be shared between threads
/// - + 'static
/// This is a lifetime bound. It indicates that the function object must have a
/// `'static` lifetime, meaning it can live for the entire duration of the
/// program. This is required because trait objects have a default lifetime
/// bound of `'static` when used with smart pointers like `Arc`.
pub type WidgetInsertClosure = Arc<dyn Fn(&mut egui::Ui, &AssetServer) + Send + Sync + 'static>;

/// PanelBuildTree Type Aliasing
/// - `BTreeMap`: Container for key-value pairs that is :
///    - Ordered by key, useful for UI elements where dock draw order matters
///    - Fast for lookups
///    - Insertion is slower when compared to HashMap
/// -  `<DockPanelLocation, DockPanelProperties>`: keys and values
pub type PanelBuildTree = BTreeMap<DockPanelLocation, DockPanelProperties>;

/// DockPanelLocation Enum for the possible types of docking panels
/// - Left
/// - Right
/// - Top
/// - Bottom
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum DockPanelLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl DockPanelLocation {
    fn label(&self) -> &str {
        match self {
            DockPanelLocation::Left => "Left Panel",
            DockPanelLocation::Right => "Right Panel",
            DockPanelLocation::Top => "Top Panel",
            DockPanelLocation::Bottom => "Bottom Panel",
        }
    }
}

#[derive(Clone)]
pub struct DockPanelProperties {
    builder: WidgetInsertClosure,
}

impl DockPanelProperties {
    pub fn new(builder: WidgetInsertClosure) -> Self {
        DockPanelProperties { builder }
    }
}

// Shared between toggle_dock and draw_dock, so it needs to be a resource
// and can't be a Local<T>
#[derive(Debug, Resource)]
struct IsVisible {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

// Set default values for IsVisible
impl Default for IsVisible {
    fn default() -> Self {
        IsVisible {
            top: false,
            bottom: false,
            left: true,
            right: true,
        }
    }
}

/* ------ Image Resources ------ */

#[derive(Debug, Clone, PartialEq, Eq)]
struct Images {
    icon: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            icon: asset_server.load("icon.png"),
        }
    }
}

/* ------ DockPlugin ------- */

// PanelBuilderHash is a closure to build a panel
pub struct DockPlugin {
    panels: PanelBuildTree,
}

impl DockPlugin {
    pub fn new(panels: PanelBuildTree) -> Self {
        DockPlugin { panels }
    }
}

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        let panels = self.panels.clone();

        app.add_plugin(EguiPlugin)
            .insert_resource(IsVisible::default())
            .insert_resource(DrawDockParams {
                panel_builders: panels,
            })
            .add_system(toggle_dock)
            .add_system(draw_dock_panels);
    }
}

#[derive(Resource)]
struct DrawDockParams {
    panel_builders: PanelBuildTree,
}

impl Default for DrawDockParams {
    fn default() -> Self {
        Self {
            panel_builders: BTreeMap::new(),
        }
    }
}

/* ------ Panel Draw Functions ------- */

/// toggles docking panels visibility
///
/// - is_visible: ResMut<IsVisible>     // is visible
/// - key_input: Res<Input<KeyCode>>    // key input
fn toggle_dock(mut is_visible: ResMut<IsVisible>, key_input: Res<Input<KeyCode>>) {
    let panel_type = if key_input.just_released(KeyCode::Left) {
        Some(DockPanelLocation::Left)
    } else if key_input.just_released(KeyCode::Right) {
        Some(DockPanelLocation::Right)
    } else if key_input.just_released(KeyCode::Up) {
        Some(DockPanelLocation::Top)
    } else if key_input.just_released(KeyCode::Down) {
        Some(DockPanelLocation::Bottom)
    } else {
        None
    };

    if let Some(panel_type) = panel_type {
        match panel_type {
            DockPanelLocation::Left => is_visible.left = !is_visible.left,
            DockPanelLocation::Right => is_visible.right = !is_visible.right,
            DockPanelLocation::Top => is_visible.top = !is_visible.top,
            DockPanelLocation::Bottom => is_visible.bottom = !is_visible.bottom,
        }
    }
}

/// draws docking panels
/// - contexts: EguiContexts            // egui context
/// - is_visible: Res<IsVisible>        // is visible
/// - params: Res<DrawDockParams>       // draw dock params
fn draw_dock_panels(
    mut contexts: EguiContexts,
    is_visible: Res<IsVisible>,
    params: Res<DrawDockParams>,
    mut texture_id: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    images: Local<Images>,
    asset_server: Res<AssetServer>,
) {
    if !*is_initialized {
        *is_initialized = true;
        *texture_id = contexts.add_image(images.icon.clone_weak());
    }

    for (panel_type, panel_data) in &params.panel_builders {
        if (*panel_type == DockPanelLocation::Left && is_visible.left)
            || (*panel_type == DockPanelLocation::Right && is_visible.right)
            || (*panel_type == DockPanelLocation::Top && is_visible.top)
            || (*panel_type == DockPanelLocation::Bottom && is_visible.bottom)
        {
            panel_builder(
                &mut contexts,
                &panel_data.builder,
                panel_type,
                panel_type.label().to_string(),
                *texture_id,
                &asset_server,
            );
        }
    }
}

/* ------ Panel Utilitie Functions ------- */

/// Returns an egui::Vec2 representing the size of the panel (to draw function)
/// - contexts: &mut Context             // the egui context to build the panel in
/// - widget_closure: &WidgetInsertClosure  // closure used to populate the panel
/// - p_type: &DockPanelLocation            // the panel type
/// - p_label: String               // a string for the panel LABEL to build
fn panel_builder(
    contexts: &mut EguiContexts,
    widget_closure: &WidgetInsertClosure,
    p_location: &DockPanelLocation,
    p_label: String,
    texture_id: egui::TextureId,
    asset_server: &Res<AssetServer>,
) -> egui::Vec2 {
    let ctx = contexts.ctx_mut();
    let ui = match *p_location {
        DockPanelLocation::Left => egui::SidePanel::left(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(egui::widgets::Image::new(texture_id, [64.0, 64.0]));
                widget_closure(ui, &*asset_server);
                let available_rect = ui.available_rect_before_wrap();
                ui.allocate_rect(available_rect, egui::Sense::hover());
            }),
        DockPanelLocation::Right => {
            egui::SidePanel::right(p_label)
                .resizable(true)
                .show(ctx, |ui| {
                    widget_closure(ui, &*asset_server);
                    let available_rect = ui.available_rect_before_wrap();
                    ui.allocate_rect(available_rect, egui::Sense::hover());
                })
        }
        DockPanelLocation::Top => {
            egui::TopBottomPanel::top(p_label)
                .resizable(true)
                .show(ctx, |ui| {
                    widget_closure(ui, &*asset_server);
                    let available_rect = ui.available_rect_before_wrap();
                    ui.allocate_rect(available_rect, egui::Sense::hover());
                })
        }
        DockPanelLocation::Bottom => {
            egui::TopBottomPanel::bottom(p_label)
                .resizable(true)
                .show(ctx, |ui| {
                    widget_closure(ui, &*asset_server);
                    let available_rect = ui.available_rect_before_wrap();
                    ui.allocate_rect(available_rect, egui::Sense::hover());
                })
        }
    };

    ui.response.rect.size()
}
