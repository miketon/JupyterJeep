use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, bevy_egui::EguiPlugin, egui};

mod dock_plugin;
use dock_plugin::DockPlugin;

#[derive(Debug, Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(DockPlugin)
        .insert_resource(OccupiedScreenSpace::default())
        .add_system(ui_dock_draw)
        .run();
}

fn ui_dock_draw(mut contexts: EguiContexts, mut oss: ResMut<OccupiedScreenSpace>) {
    let ctx = contexts.ctx_mut();
    oss.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    oss.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    oss.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Top Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    oss.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}
