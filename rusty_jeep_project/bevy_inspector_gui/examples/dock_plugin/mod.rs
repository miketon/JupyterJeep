use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::collections::HashMap;
use std::sync::Arc;

/// PanelBuilder Type Aliasing Complex Traits
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
pub type PanelBuilder = Arc<dyn Fn(&mut egui::Ui, &AssetServer) + Send + Sync + 'static>;

/// PanelBuilderHash Type Aliasing
/// - `HashMap`: This is a container that can store and organize pieces of  
/// information called "key-value pairs." It lets us quickly find a value based  
/// on its key, like using a word in a dictionary to find its definition.
/// -  `<PanelType, PanelData>`: These are the types of keys and values that the
/// `HashMap` can store. `PanelType` is the type of key, and `PanelData` is the
/// type of value.
pub type PanelBuilderHash = HashMap<PanelType, PanelData>;

/// PanelType Enum for the possible types of docking panels
/// - Left
/// - Right
/// - Top
/// - Bottom
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PanelType {
    Left,
    Right,
    Top,
    Bottom,
}

impl PanelType {
    fn label(&self) -> &str {
        match self {
            PanelType::Left => "Left Panel",
            PanelType::Right => "Right Panel",
            PanelType::Top => "Top Panel",
            PanelType::Bottom => "Bottom Panel",
        }
    }
}

#[derive(Clone)]
pub struct PanelData {
    builder: PanelBuilder,
}

impl PanelData {
    pub fn new(builder: PanelBuilder) -> Self {
        PanelData { builder }
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

#[derive(Debug, Default, Resource)]
struct OccupiedSpace {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
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
    panels: PanelBuilderHash,
}

impl DockPlugin {
    pub fn new(panels: PanelBuilderHash) -> Self {
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
            .add_system(draw_dock);
    }
}

#[derive(Resource)]
struct DrawDockParams {
    panel_builders: PanelBuilderHash,
}

impl Default for DrawDockParams {
    fn default() -> Self {
        Self {
            panel_builders: HashMap::new(),
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
        Some(PanelType::Left)
    } else if key_input.just_released(KeyCode::Right) {
        Some(PanelType::Right)
    } else if key_input.just_released(KeyCode::Up) {
        Some(PanelType::Top)
    } else if key_input.just_released(KeyCode::Down) {
        Some(PanelType::Bottom)
    } else {
        None
    };

    if let Some(panel_type) = panel_type {
        match panel_type {
            PanelType::Left => is_visible.left = !is_visible.left,
            PanelType::Right => is_visible.right = !is_visible.right,
            PanelType::Top => is_visible.top = !is_visible.top,
            PanelType::Bottom => is_visible.bottom = !is_visible.bottom,
        }
    }
}

/// draws docking panesl
/// - contexts: EguiContexts            // egui context
/// - is_visible: Res<IsVisible>        // is visible
/// - params: Res<DrawDockParams>       // draw dock params
/// - mut o_space: Local<OccupiedSpace> // occupied space
fn draw_dock(
    mut contexts: EguiContexts,
    is_visible: Res<IsVisible>,
    params: Res<DrawDockParams>,
    mut o_space: Local<OccupiedSpace>,
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
        if (*panel_type == PanelType::Left && is_visible.left)
            || (*panel_type == PanelType::Right && is_visible.right)
            || (*panel_type == PanelType::Top && is_visible.top)
            || (*panel_type == PanelType::Bottom && is_visible.bottom)
        {
            let size = panel_builder(
                &mut contexts,
                &panel_data.builder,
                panel_type,
                panel_type.label().to_string(),
                *texture_id,
                &asset_server,
            );
            match *panel_type {
                PanelType::Left => {
                    o_space.left = size.x;
                }
                PanelType::Right => {
                    o_space.right = size.x;
                }
                PanelType::Top => {
                    o_space.top = size.y;
                }
                PanelType::Bottom => {
                    o_space.bottom = size.y;
                }
            }
        }
    }
}

/* ------ Panel Utilitie Functions ------- */

/// Returns an egui::Vec2 representing the size of the panel (to draw function)
/// - ctx: &mut Context             // the egui context to build the panel in
/// - panel_builder: &PanelBuilder  // closure used to populate the panel
/// - p_type: &PanelType            // the panel type
/// - p_label: String               // a string for the panel LABEL to build
fn panel_builder(
    contexts: &mut EguiContexts,
    panel_builder: &PanelBuilder,
    p_type: &PanelType,
    p_label: String,
    texture_id: egui::TextureId,
    asset_server: &Res<AssetServer>,
) -> egui::Vec2 {
    let ctx = contexts.ctx_mut();
    let ui = match *p_type {
        PanelType::Left => egui::SidePanel::left(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(egui::widgets::Image::new(texture_id, [64.0, 64.0]));
                panel_builder(ui, &*asset_server);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            }),
        PanelType::Right => egui::SidePanel::right(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui, &*asset_server);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            }),
        PanelType::Top => egui::TopBottomPanel::top(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui, &*asset_server);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            }),
        PanelType::Bottom => {
            egui::TopBottomPanel::bottom(p_label)
                .resizable(true)
                .show(ctx, |ui| {
                    panel_builder(ui, &*asset_server);
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                })
        }
    };

    ui.response.rect.size()
}
