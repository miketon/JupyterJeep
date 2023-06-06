use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, bevy_egui::EguiPlugin, egui, egui::Context};

enum PanelBuildType {
    Side(egui::SidePanel),
    TopBottom(egui::TopBottomPanel),
}

#[derive(Debug, Resource, Default)]
struct OccupiedSpace {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Debug, Resource, Default)]
struct IsVisible {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

pub struct DockPlugin;

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .insert_resource(OccupiedSpace::default())
            .insert_resource(IsVisible::default())
            .add_system(toggle_dock)
            .add_system(draw_dock)
            .add_system(ui_example_system);
    }
}

fn toggle_dock(mut is_visible: ResMut<IsVisible>, key_input: Res<Input<KeyCode>>) {
    match () {
        _ if key_input.just_released(KeyCode::Left) => is_visible.left = !is_visible.left,
        _ if key_input.just_released(KeyCode::Right) => is_visible.right = !is_visible.right,
        _ if key_input.just_released(KeyCode::Up) => is_visible.top = !is_visible.top,
        _ if key_input.just_released(KeyCode::Down) => is_visible.bottom = !is_visible.bottom,
        _ => (), // No-op if no relevant key was pressed
    }
}

fn draw_dock(
    mut contexts: EguiContexts,
    mut o_space: ResMut<OccupiedSpace>,
    is_visible: ResMut<IsVisible>,
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

fn panel_builder(ctx: &mut Context, p_type: &str, p_label: String) -> egui::Vec2 {
    let panel_builder = match p_type {
        "left" => PanelBuildType::Side(egui::SidePanel::left(p_label.clone())),
        "right" => PanelBuildType::Side(egui::SidePanel::right(p_label.clone())),
        "top" => PanelBuildType::TopBottom(egui::TopBottomPanel::top(p_label.clone())),
        "bottom" => PanelBuildType::TopBottom(egui::TopBottomPanel::bottom(p_label.clone())),
        _ => panic!("Invalid panel type"),
    };

    let response = match panel_builder {
        PanelBuildType::Side(side_panel) => {
            side_panel
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label(p_label);
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
                })
                .response
        }
        PanelBuildType::TopBottom(top_bottom_panel) => {
            top_bottom_panel
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label(p_label);
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
                })
                .response
        }
    };

    response.rect.size()
}

fn ui_example_system(mut ctx: EguiContexts) {
    egui::Window::new("DockPlugin").show(ctx.ctx_mut(), |ui| {
        ui.label("Don't Eat Reynolds Wraps Ovie!");
    });
}
