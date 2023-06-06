use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, bevy_egui::EguiPlugin, egui};

#[derive(Debug, Resource, Default)]
struct OccupiedScreenSpace {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub struct DockPlugin;

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OccupiedScreenSpace::default())
            .add_system(draw_dock)
            .add_system(ui_example_system);
    }
}

fn draw_dock() {
    println!("draw_dock");
}

fn ui_example_system(mut ctx: EguiContexts) {
    egui::Window::new("DockPlugin").show(ctx.ctx_mut(), |ui| {
        ui.label("Don't Eat Reynolds Wraps Ovie!");
    });
}
