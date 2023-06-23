use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::epaint::{Mesh, Vertex, WHITE_UV};
use bevy_inspector_egui::bevy_egui::egui::{self, Color32, Rect, Sense, Shape, Vec2};
use bevy_inspector_egui::egui::emath::RectTransform;
use bevy_inspector_egui::egui::Pos2;
use std::collections::BTreeMap;
use std::sync::Arc;

mod dock_plugin;
use dock_plugin::{DockClosure, DockLocation, DockPlugin};

fn main() {
    let mut panel_tree = BTreeMap::new();
    panel_tree.insert(
        DockLocation::Left,
        DockClosure::new(Arc::new(|ui: &mut egui::Ui, asset_server| {
            ui.label("Left Panel");
            ui.horizontal(|ui| {
                ui.allocate_space(Vec2::new(20.0, 0.0));
                ui_triangle_mesh(ui, 6.0);
            });
            ui.horizontal(|ui| {
                ui_triangle_mesh(ui, 6.0);
                ui_triangle_mesh(ui, 6.0);
            });
            ui.vertical(|ui| {
                ui.allocate_space(Vec2::new(0.0, 5.0));
                ui.separator();
                ui.horizontal(|ui| {
                    ui.allocate_space(Vec2::new(10.0, 0.0));
                    ui_counter_widget(ui);
                    ui.separator();
                    ui.label("--");
                });
                ui.separator();
            });
        })),
    );
    panel_tree.insert(
        DockLocation::Bottom,
        DockClosure::new(Arc::new(|ui: &mut egui::Ui, asset_server| {
            ui.label("Bottom Panel");
            ui_animator_widget(ui);
        })),
    );

    let dock_plugin = DockPlugin::new(panel_tree);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(dock_plugin)
        .run();
}

fn ui_animator_widget(ui: &mut egui::Ui) {
    ui.label("Animator");
}

fn ui_triangle_mesh(ui: &mut egui::Ui, scale: f32) {
    let (response, painter) = ui.allocate_painter(Vec2 { x: 50.0, y: 50.0 }, Sense::hover());

    let to_screen = RectTransform::from_to(
        Rect::from_min_size(Pos2::ZERO, scale * response.rect.size()),
        response.rect,
    );
    let v1 = to_screen.transform_pos(Pos2 { x: 200.0, y: 100.0 });
    let v2 = to_screen.transform_pos(Pos2 { x: 100.0, y: 300.0 });
    let v3 = to_screen.transform_pos(Pos2 { x: 300.0, y: 300.0 });
    let vertices = vec![
        Vertex {
            pos: v1,
            color: Color32::RED,
            uv: WHITE_UV,
        },
        Vertex {
            pos: v2,
            color: Color32::GREEN,
            uv: WHITE_UV,
        },
        Vertex {
            pos: v3,
            color: Color32::BLUE,
            uv: WHITE_UV,
        },
    ];

    let indices = vec![0, 1, 2];
    let mesh = Mesh {
        vertices,
        indices,
        ..Default::default()
    };

    painter.add(Shape::Mesh(mesh));
}

fn ui_counter_widget(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            println!("Decrease counter");
        }
        ui.label("Counter");
        if ui.button("+").clicked() {
            println!("Increase counter");
        }
    });
}
