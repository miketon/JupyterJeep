use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, bevy_egui::EguiPlugin, egui, egui::Context};

enum PanelBuildType {
    Side(egui::SidePanel),
    TopBottom(egui::TopBottomPanel),
}

#[derive(Debug, Default)]
struct OccupiedSpace {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

// Shared between toggle_dock and draw_dock, so it needs to be a resource
// and can't be a Local<T>
#[derive(Debug, Resource)]
struct IsVisible {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

// Set default values for IsVisible
impl Default for IsVisible {
    fn default() -> Self {
        IsVisible {
            left: true,
            right: true,
            top: true,
            bottom: true,
        }
    }
}

/* ------ DockPlugin ------- */

pub struct DockPlugin;

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .insert_resource(IsVisible::default())
            .add_system(toggle_dock)
            .add_system(draw_dock)
            .add_system(ui_example_system);
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
/// - o_space: ResMut<OccupiedSpace>    // occupied space
/// - is_visible: Res<IsVisible>        // is visible
fn draw_dock(
    mut contexts: EguiContexts,
    mut o_space: Local<OccupiedSpace>,
    is_visible: Res<IsVisible>,
) {
    let ctx = contexts.ctx_mut();
    if is_visible.left {
        o_space.left = panel_builder(ctx, "left", "Left Panel".to_string()).x;
    }
    if is_visible.right {
        o_space.right = panel_builder(ctx, "right", "Right Panel".to_string()).x;
    }
    if is_visible.top {
        o_space.top = panel_builder(ctx, "top", "Top Panel".to_string()).y;
    }
    if is_visible.bottom {
        o_space.bottom = panel_builder(ctx, "bottom", "Bottom Panel".to_string()).y;
    }
}

/* ------ Panel Utilitie Functions ------- */

/// Returns an egui::Vec2 representing the size of the panel
/// - ctx: &mut Context // the egui context to build the panel in
/// - p_type: &str      // a string slice for the panel TYPE to build
/// - p_label: String   // a string for the panel LABEL to build
fn panel_builder(ctx: &mut Context, p_type: &str, p_label: String) -> egui::Vec2 {
    let panel_builder = match p_type {
        "left" => PanelBuildType::Side(egui::SidePanel::left(p_label.clone())),
        "right" => PanelBuildType::Side(egui::SidePanel::right(p_label.clone())),
        "top" => PanelBuildType::TopBottom(egui::TopBottomPanel::top(p_label.clone())),
        "bottom" => PanelBuildType::TopBottom(egui::TopBottomPanel::bottom(p_label.clone())),
        _ => panic!("Invalid panel type"),
    };

    // match expression to create an egui::Response based on PanelBuildType enum
    // -
    let response = match panel_builder {
        PanelBuildType::Side(side_panel) => {
            side_panel
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label(&p_label);
                    // allocate layout space for the panel
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
                })
                .response
        }
        PanelBuildType::TopBottom(top_bottom_panel) => {
            top_bottom_panel
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label(&p_label);
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
                })
                .response
        }
    };

    // extract the RECT size of the panel from the response
    // egui::Response represents the result of interacting with a user interface
    // (UI) element. It contains information about the current state of the
    // interaction, such as whether the element was clicked, hovered, or
    // dragged, along with other details like the position and size of the
    // element.
    response.rect.size()
}

fn ui_example_system(mut ctx: EguiContexts) {
    egui::Window::new("DockPlugin").show(ctx.ctx_mut(), |ui| {
        ui.label("Don't Eat Reynolds Wraps Ovie!");
    });
}
