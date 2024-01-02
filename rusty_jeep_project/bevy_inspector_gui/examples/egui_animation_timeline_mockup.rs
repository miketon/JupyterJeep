use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::epaint::{
    Mesh, RectShape, Rounding, Stroke, Vertex, WHITE_UV,
};
use bevy_inspector_egui::bevy_egui::egui::{
    self, pos2, vec2, Color32, Painter, Pos2, Rect, Response, Sense, Shape, Vec2,
};

use bevy_inspector_egui::egui::emath::RectTransform;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

mod dock_plugin;
use dock_plugin::{DockClosure, DockLocation, DockPlugin};

mod debug_text_plugin;
use debug_text_plugin::DebugTextPlugin;

#[derive(Debug, Resource)]
struct TimeLineState {
    is_dragging: bool,
    // Where did user start dragging, used to calculate distance dragged
    drag_start: Pos2, // @note : has to be type... can't be pos2() function
    // Position where the clip is released
    position: Pos2,
}

impl Default for TimeLineState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            drag_start: pos2(0.0, 0.0),
            position: pos2(0.0, 0.0),
        }
    }
}

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

    let timeline_state = Arc::new(Mutex::new(TimeLineState::default()));

    panel_tree.insert(
        DockLocation::Bottom,
        DockClosure::new(Arc::new(move |ui: &mut egui::Ui| {
            // access ResMut<TimeLineState> here ..how?
            ui.label("Bottom Panel");
            let timeline_state_clone = timeline_state.clone();
            animator_timeline_panel(ui, timeline_state_clone);
        })),
    );

    let dock_plugin = DockPlugin::new(panel_tree);
    let debug_text_plugin = DebugTextPlugin::new();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(dock_plugin)
        .add_plugin(debug_text_plugin)
        .run();
}

/// Draw an Animator TimeLine Panel
/// @param ui
/// @param timeline_state
fn animator_timeline_panel(ui: &mut egui::Ui, timeline_state: Arc<Mutex<TimeLineState>>) {
    const TIMELINE_HEIGHT: f32 = 200.0;
    let timeline_width: f32 = ui.available_width();
    ui.label("Animation Timeline");
    // Draw squares representation the animations
    let (painter, to_screen, response) =
        allocate_painter(ui, vec2(timeline_width, TIMELINE_HEIGHT));
    draw_animator_ticks(&painter, &to_screen, timeline_width);

    let key_min = to_screen.transform_pos(Pos2::ZERO);
    let key_max = to_screen.transform_pos(Pos2 { x: 200.0, y: 100.0 });
    draw_key_frame(&painter, &key_min, &key_max);
    // draw_clip_button(ui, &to_screen);
    let mut timeline_state = match timeline_state.lock() {
        Ok(unlocked) => unlocked,
        Err(e) => {
            eprint!("Failed to lock timeline state {:?}", e);
            return;
        }
    };

    let y_ss = to_screen.transform_pos(pos2(0.0, 0.0));
    let x_ss = to_screen.transform_pos(timeline_state.position);
    let animation_clip_button = ui.put(
        Rect {
            min: pos2(x_ss.x, y_ss.y + 10.0),
            max: pos2(x_ss.x + 300.0, y_ss.y + 90.0),
        },
        egui::Button::new("Square").sense(Sense::drag()),
    );

    let mut debug_text_update = String::new();

    if animation_clip_button.drag_started() {
        let drag_start = animation_clip_button.interact_pointer_pos();
        match drag_start {
            Some(start_pos) => {
                debug_text_update.push_str("Drag Started");
                timeline_state.drag_start = start_pos;
                timeline_state.is_dragging = true;
                // // reset drag delta on drag start
                // timeline_state.drag_delta = 0.0;
            }
            None => {}
        }
    }

    if timeline_state.is_dragging {
        let drag_delta = animation_clip_button.interact_pointer_pos();
        match drag_delta {
            Some(delta_pos) => {
                debug_text_update.push_str("Is Dragging");
                // Calculate the difference between the last stored position
                let drag_delta = delta_pos.x - timeline_state.drag_start.x;
                // Update the clip position in state
                timeline_state.position.x = drag_delta + timeline_state.position.x;
                // Update the last stored position the mouse's current position
                // Since we are updating each tick, this makes the drag seamless
                timeline_state.drag_start.x = delta_pos.x;
            }
            None => {}
        }
    }

    if animation_clip_button.drag_released() {
        debug_text_update.push_str("Drag Released");
        // On release, reset the drag start
        timeline_state.drag_start = pos2(0.0, 0.0);
        timeline_state.is_dragging = false;
    }

    debug_text_update.push_str(format!("{:?}", timeline_state).as_str());

    // Time line scrubber
    if response.hovered() {
        let hovered_pos = response.hover_pos();
        match hovered_pos {
            Some(pos) => {
                // we only want y to be screen space???
                // @audit - understand this better
                let relative_pos = to_screen.transform_pos(Pos2::new(pos.x, 0.0));
                let first_point = pos2(pos.x, relative_pos.y);
                let second_point = pos2(pos.x, relative_pos.y + 100.0);
                draw_line(&painter, &first_point, &second_point, Color32::WHITE);
                ui.separator();
                ui.label(format!("Hovered {:?}", pos));
            }
            None => {}
        }
    }

    // Create a separate window for the debug text
    egui::Window::new("Debug Text").show(ui.ctx(), |ui| {
        ui.label(debug_text_update);
    });
}

/// Loop to draw vertical animation ticks
/// @param painter - the painter to draw with
/// @param to_screen - the transform to screen space
/// @param timeline_length - the length of the timeline
fn draw_animator_ticks(painter: &Painter, to_screen: &RectTransform, timeline_length: f32) {
    let timeline_line_gap = 20.0;
    let timeline_num_lines: i32 = (timeline_length / timeline_line_gap).round() as i32;

    // Loop over the number of lines we need
    for index in 0..timeline_num_lines {
        // Then we scale using the gap length
        let x = (index as f32) * timeline_line_gap;

        // Create two points for the timeline segment in SCREEN SPACE
        let first_point = to_screen.transform_pos(Pos2 { x, y: 0.0 });
        let second_point = to_screen.transform_pos(Pos2 { x, y: 100.0 });

        // Draw a vertical line
        draw_line(&painter, &first_point, &second_point, Color32::DARK_GRAY);
    }
}

/// Draw a line segment
///
/// # Arguments
/// * `painter` - The painter to draw the line on.
/// * `first_point` - The first point of the line segment.
/// * `second_point` - The second point of the line segment.
/// * `color` - The color of the line segment.
///
/// # Note
/// Pos2 is passed as a reference to be more efficient than copying.
fn draw_line(painter: &Painter, first_point: &Pos2, second_point: &Pos2, color: Color32) {
    painter.add(Shape::LineSegment {
        points: [*first_point, *second_point],
        stroke: Stroke {
            width: 1.0,
            color,
        },
    });
}

/// draw a key frame
/// @param painter - the painter to draw the key frame on
/// @param key_min - the min point of the key frame
/// @param key_max - the max point of the key frame
fn draw_key_frame(painter: &Painter, key_min: &Pos2, key_max: &Pos2) {
    painter.add(Shape::Rect(RectShape {
        rect: Rect {
            min: *key_min,
            max: *key_max,
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

/// draw a triangle mesh - throwaway function for testing
/// @param painter - the painter to draw the triangle mesh on
/// @param v1 - the first vertex of the triangle mesh
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

/// draw a counter widget - throwaway function for testing
/// @param ui - the ui to draw the counter widget on
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

/// allocate a painter
/// @param ui - the ui to allocate the painter on
/// @param size - the size of the painter
/// @return painter - the painter
/// @return to_screen - the rect transform to screen space
/// @return response - the response of the painter
fn allocate_painter(ui: &mut egui::Ui, size: Vec2) -> (Painter, RectTransform, Response) {
    let (response, painter) = ui.allocate_painter(size, Sense::hover());
    let to_screen = RectTransform::from_to(
        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
        response.rect,
    );
    (painter, to_screen, response)
}
