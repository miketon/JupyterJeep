use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::epaint::{
    Mesh, RectShape, Rounding, Stroke, Vertex, WHITE_UV,
};
use bevy_inspector_egui::bevy_egui::egui::{
    self, vec2, Color32, Painter, Pos2, Rect, Sense, Shape, Vec2,
};

use bevy_inspector_egui::egui::emath::RectTransform;
use std::collections::BTreeMap;
use std::sync::Arc;

mod dock_plugin;
use dock_plugin::{DockClosure, DockLocation, DockPlugin};

fn main() {
    let mut panel_tree = BTreeMap::new();
    panel_tree.insert(
        DockLocation::Left,
        DockClosure::new(Arc::new(|ui: &mut egui::Ui| {
            ui.label("Left Panel");
            ui.horizontal(|ui| {
                ui.allocate_space(vec2(20.0, 0.0));
                ui_triangle_mesh(ui, 6.0);
            });
            ui.horizontal(|ui| {
                ui_triangle_mesh(ui, 6.0);
                ui_triangle_mesh(ui, 6.0);
            });
            ui.vertical(|ui| {
                ui.allocate_space(vec2(0.0, 5.0));
                ui.separator();
                ui.horizontal(|ui| {
                    ui.allocate_space(vec2(10.0, 0.0));
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
        DockClosure::new(Arc::new(|ui: &mut egui::Ui| {
            ui.label("Bottom Panel");
            animator_timeline_panel(ui);
        })),
    );

    let dock_plugin = DockPlugin::new(panel_tree);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(dock_plugin)
        .run();
}

fn animator_timeline_panel(ui: &mut egui::Ui) {
    ui.label("Animation Timeline");
    let timeline_width = ui.available_width();
    // Draw squares representation the animations
    let (painter, to_screen) = allocate_painter(ui, vec2(timeline_width, 100.0));
    draw_animator_ticks(&painter, &to_screen, timeline_width);

    let key_min = to_screen.transform_pos(Pos2::ZERO);
    let key_max = to_screen.transform_pos(Pos2 { x: 200.0, y: 100.0 });
    draw_key_frame(&painter, key_min, key_max);
}

fn draw_animator_ticks(painter: &Painter, to_screen: &RectTransform, timeline_length: f32) {
    let timeline_line_gap = 20.0;
    let timeline_num_lines: i32 = (timeline_length / timeline_line_gap).round() as i32;

    // loop over the number of lines we need
    for index in 0..timeline_num_lines {
        // then we scale using the gap length
        let x = (index as f32) * timeline_line_gap;

        // Create our 2 points for the timelin segment
        // And transform it to screen space
        let first_point = to_screen.transform_pos(Pos2 { x, y: 0.0 });
        let second_point = to_screen.transform_pos(Pos2 { x, y: 100.0 });

        // Draw a vertical line
        draw_line(&painter, first_point, second_point);
    }
}

fn draw_line(painter: &Painter, first_point: Pos2, second_point: Pos2) {
    painter.add(Shape::LineSegment {
        points: [first_point, second_point],
        stroke: Stroke {
            width: 1.0,
            color: Color32::DARK_GRAY,
        },
    });
}

fn draw_key_frame(painter: &Painter, key_min: Pos2, key_max: Pos2) {
    painter.add(Shape::Rect(RectShape {
        rect: Rect {
            min: key_min,
            max: key_max,
        },
        rounding: Rounding {
            nw: 0.0,
            ne: 0.0,
            se: 0.0,
            sw: 0.0,
        },
        fill: Color32::BLUE,
        stroke: Stroke {
            width: 2.0,
            color: Color32::WHITE,
        },
    }));
}

fn ui_triangle_mesh(ui: &mut egui::Ui, scale: f32) {
    let (response, painter) = ui.allocate_painter(vec2(50.0, 50.0), Sense::hover());

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

fn allocate_painter(ui: &mut egui::Ui, size: Vec2) -> (Painter, RectTransform) {
    let (response, painter) = ui.allocate_painter(size, Sense::hover());
    let to_screen = RectTransform::from_to(
        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
        response.rect,
    );
    (painter, to_screen)
}
