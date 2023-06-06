use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, bevy_egui::EguiPlugin, egui, egui::Context};

#[derive(Debug, Resource, Default)]
struct OccupiedScreenSpace {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    left_is_visible: bool,
    right_is_visible: bool,
    top_is_visible: bool,
    bottom_is_visible: bool,
}

pub struct DockPlugin;

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .insert_resource(OccupiedScreenSpace::default())
            .add_system(toggle_dock)
            .add_system(draw_dock)
            .add_system(ui_example_system);
    }
}

fn toggle_dock(mut oss: ResMut<OccupiedScreenSpace>, key_input: Res<Input<KeyCode>>) {
    match () {
        _ if key_input.just_released(KeyCode::Left) => oss.left_is_visible = !oss.left_is_visible,
        _ if key_input.just_released(KeyCode::Right) => {
            oss.right_is_visible = !oss.right_is_visible
        }
        _ if key_input.just_released(KeyCode::Up) => oss.top_is_visible = !oss.top_is_visible,
        _ if key_input.just_released(KeyCode::Down) => {
            oss.bottom_is_visible = !oss.bottom_is_visible
        }
        _ => (), // No-op if no relevant key was pressed
    }
}

fn draw_dock(mut contexts: EguiContexts, mut oss: ResMut<OccupiedScreenSpace>) {
    let ctx = contexts.ctx_mut();
    left_panel(ctx, &mut oss);
    right_panel(ctx, &mut oss);
    top_panel(ctx, &mut oss);
    bottom_panel(ctx, &mut oss);
}

fn left_panel(ctx: &mut Context, oss: &mut OccupiedScreenSpace) {
    if oss.left_is_visible {
        oss.left = egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Left Panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
            })
            .response
            .rect
            .width()
    }
}

fn right_panel(ctx: &mut Context, oss: &mut OccupiedScreenSpace) {
    if oss.right_is_visible {
        oss.right = egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Right Panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
            })
            .response
            .rect
            .width()
    }
}

fn top_panel(ctx: &mut Context, oss: &mut OccupiedScreenSpace) {
    if oss.top_is_visible {
        oss.top = egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Top Panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
            })
            .response
            .rect
            .height()
    }
}

fn bottom_panel(ctx: &mut Context, oss: &mut OccupiedScreenSpace) {
    if oss.bottom_is_visible {
        oss.bottom = egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Bottom Panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
            })
            .response
            .rect
            .height()
    }
}

fn ui_example_system(mut ctx: EguiContexts) {
    egui::Window::new("DockPlugin").show(ctx.ctx_mut(), |ui| {
        ui.label("Don't Eat Reynolds Wraps Ovie!");
    });
}
