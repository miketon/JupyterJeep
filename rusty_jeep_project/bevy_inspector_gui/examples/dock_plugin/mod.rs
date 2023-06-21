use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::collections::BTreeMap;
use std::fmt;
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
pub type WidgetClosure = Arc<dyn Fn(&mut egui::Ui, &AssetServer) + Send + Sync + 'static>;

/// PanelTree Type Aliasing
/// - `BTreeMap`: Container for key-value pairs that is :
///    - Ordered by key, useful for UI elements where dock draw order matters
///    - Fast for lookups
///    - Insertion is slower when compared to HashMap
/// -  `<DockLocation, DockClosure>`: keys and values
pub type DockTree = BTreeMap<DockLocation, DockClosure>;

/// DockLocation Enum for the possible location docking panels
/// - Left
/// - Right
/// - Top
/// - Bottom
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum DockLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl fmt::Display for DockLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DockLocation::Left => write!(f, "Left Panel"),
            DockLocation::Right => write!(f, "Right Panel"),
            DockLocation::Top => write!(f, "Top Panel"),
            DockLocation::Bottom => write!(f, "Bottom Panel"),
        }
    }
}

#[derive(Clone)]
pub struct DockClosure {
    widget_closure: WidgetClosure,
}

impl DockClosure {
    pub fn new(closure: WidgetClosure) -> Self {
        DockClosure {
            widget_closure: closure,
        }
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
    dock_tree: DockTree,
}

impl DockPlugin {
    pub fn new(dock_tree: DockTree) -> Self {
        DockPlugin { dock_tree }
    }
}

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        let dock_tree = self.dock_tree.clone();

        app.add_plugin(EguiPlugin)
            .insert_resource(IsVisible::default())
            .insert_resource(DrawDockParams { dock_tree })
            .add_system(toggle_docks)
            .add_system(draw_docking_panels);
    }
}

#[derive(Resource)]
struct DrawDockParams {
    dock_tree: DockTree,
}

impl Default for DrawDockParams {
    fn default() -> Self {
        Self {
            dock_tree: BTreeMap::new(),
        }
    }
}

/* ------ Panel Draw Functions ------- */

/// toggles docking panels visibility
///
/// - is_visible: ResMut<IsVisible>     // is visible
/// - key_input: Res<Input<KeyCode>>    // key input
fn toggle_docks(mut is_visible: ResMut<IsVisible>, key_input: Res<Input<KeyCode>>) {
    let panel_type = if key_input.just_released(KeyCode::Left) {
        Some(DockLocation::Left)
    } else if key_input.just_released(KeyCode::Right) {
        Some(DockLocation::Right)
    } else if key_input.just_released(KeyCode::Up) {
        Some(DockLocation::Top)
    } else if key_input.just_released(KeyCode::Down) {
        Some(DockLocation::Bottom)
    } else {
        None
    };

    if let Some(panel_type) = panel_type {
        match panel_type {
            DockLocation::Left => is_visible.left = !is_visible.left,
            DockLocation::Right => is_visible.right = !is_visible.right,
            DockLocation::Top => is_visible.top = !is_visible.top,
            DockLocation::Bottom => is_visible.bottom = !is_visible.bottom,
        }
    }
}

/// draws docking panels
/// - contexts: EguiContexts            // egui context
/// - is_visible: Res<IsVisible>        // is visible
/// - params: Res<DrawDockParams>       // draw dock params
fn draw_docking_panels(
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

    let should_show_panel = |location: &DockLocation| -> bool {
        match location {
            DockLocation::Left => is_visible.left,
            DockLocation::Right => is_visible.right,
            DockLocation::Top => is_visible.top,
            DockLocation::Bottom => is_visible.bottom,
        }
    };

    for (dock_location, dock_properties) in &params.dock_tree {
        if should_show_panel(dock_location) {
            show_panel(
                &mut contexts,
                &dock_properties.widget_closure,
                dock_location,
                dock_location.to_string(),
                *texture_id,
                &asset_server,
            );
        }
    }
}

/* ------ Panel Utilitie Functions ------- */

/// Shows a panel
/// - contexts: &mut Context             // the egui context to build the panel in
/// - widget_closure: &WidgetInsertClosure  // closure used to populate the panel
/// - p_type: &DockPanelLocation            // the panel type
/// - p_label: String               // a string for the panel LABEL to build
fn show_panel(
    contexts: &mut EguiContexts,
    widget_closure: &WidgetClosure,
    p_location: &DockLocation,
    p_label: String,
    texture_id: egui::TextureId,
    asset_server: &Res<AssetServer>,
) {
    let ctx = contexts.ctx_mut();
    match *p_location {
        DockLocation::Left => egui::SidePanel::left(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(egui::widgets::Image::new(texture_id, [64.0, 64.0]));
                widget_closure(ui, &*asset_server);
                let available_rect = ui.available_rect_before_wrap();
                ui.allocate_rect(available_rect, egui::Sense::hover());
            }),
        DockLocation::Right => egui::SidePanel::right(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                widget_closure(ui, &*asset_server);
                let available_rect = ui.available_rect_before_wrap();
                ui.allocate_rect(available_rect, egui::Sense::hover());
            }),
        DockLocation::Top => egui::TopBottomPanel::top(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                widget_closure(ui, &*asset_server);
                let available_rect = ui.available_rect_before_wrap();
                ui.allocate_rect(available_rect, egui::Sense::hover());
            }),
        DockLocation::Bottom => {
            egui::TopBottomPanel::bottom(p_label)
                .resizable(true)
                .show(ctx, |ui| {
                    widget_closure(ui, &*asset_server);
                    let available_rect = ui.available_rect_before_wrap();
                    ui.allocate_rect(available_rect, egui::Sense::hover());
                })
        }
    };
}
