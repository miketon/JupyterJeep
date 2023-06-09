use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, bevy_egui::EguiPlugin, egui, egui::Context};
use core::panic;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
struct OccupiedSpace {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
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
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }
}

type PanelBuilder = Arc<dyn Fn(&mut egui::Ui) + Send + Sync + 'static>;
type PanelBuilderHash = HashMap<String, PanelBuilder>;

/* ------ DockPlugin ------- */

// @note : Add a field to `DockPlugin` to store the closure:
pub struct DockPlugin {
    panel_builders: PanelBuilderHash,
}
// @note :  Update the `DockPlugin` implementation to store the closure in the plugin
impl DockPlugin {
    pub fn new(
        panel_builders: PanelBuilderHash,
    ) -> Self {
        DockPlugin { panel_builders }
    }
}

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        let panel_builders = self.panel_builders.clone();

        app.add_plugin(EguiPlugin)
            .insert_resource(IsVisible::default())
            .insert_resource(DrawDockParams { panel_builders })
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
fn toggle_dock(mut is_visible: ResMut<IsVisible>, key_input: Res<Input<KeyCode>>) {
    // if else exhaustive comparison as opposed to match because
    // - forcing factor to handle new cases when added
    if key_input.just_released(KeyCode::Left) {
        is_visible.left = !is_visible.left;
    } else if key_input.just_released(KeyCode::Right) {
        is_visible.right = !is_visible.right;
    } else if key_input.just_released(KeyCode::Up) {
        is_visible.top = !is_visible.top;
    } else if key_input.just_released(KeyCode::Down) {
        is_visible.bottom = !is_visible.bottom;
    }
}

/// draws docking panesl
/// - contexts: EguiContexts            // egui context
/// - is_visible: Res<IsVisible>        // is visible
/// - params: Res<DrawDockParams>       // draw dock params
fn draw_dock(mut contexts: EguiContexts, is_visible: Res<IsVisible>, params: Res<DrawDockParams>) {
    let ctx = contexts.ctx_mut();
    let mut o_space = OccupiedSpace::default();

    for (panel_type, p_builder) in &params.panel_builders {
        let p_label = format!("{} Panel", panel_type.to_uppercase());

        if (panel_type == "top" && is_visible.top)
            || (panel_type == "bottom" && is_visible.bottom)
            || (panel_type == "left" && is_visible.left)
            || (panel_type == "right" && is_visible.right)
        {
            let size = panel_builder(ctx, p_builder, panel_type, p_label);

            match panel_type.as_str() {
                "top" => {
                    o_space.top = size.y;
                }
                "bottom" => {
                    o_space.bottom = size.y;
                }
                "left" => {
                    o_space.left = size.x;
                }
                "right" => {
                    o_space.right = size.x;
                }
                _ => panic!("Invalid panel type {}", panel_type),
            }
        }
    }
}

/* ------ Panel Utilitie Functions ------- */

/// Returns an egui::Vec2 representing the size of the panel
/// - ctx: &mut Context // the egui context to build the panel in
/// // the panel builder closure
/// - panel_builder: &Arc<dyn Fn(&mut egui::Ui) + Send + Sync + 'static>
/// - p_type: &str      // a string slice for the panel TYPE to build
/// - p_label: String   // a string for the panel LABEL to build
fn panel_builder(
    ctx: &mut Context,
    panel_builder: &PanelBuilder,
    p_type: &str,
    p_label: String,
) -> egui::Vec2 {
    let ui = match p_type {
        "top" => egui::TopBottomPanel::top(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui);
            }),
        "bottom" => egui::TopBottomPanel::bottom(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui);
            }),
        "left" => egui::SidePanel::left(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui);
            }),
        "right" => egui::SidePanel::right(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui);
            }),
        _ => panic!("Invalid panel type {}", p_type),
    };

    ui.response.rect.size()
}
