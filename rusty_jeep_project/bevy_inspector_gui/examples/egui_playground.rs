use bevy::prelude::*;
use bevy_inspector_egui::egui::emath::RectTransform;
use bevy_inspector_egui::egui::{self, Pos2, Rect, Sense, Vec2};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

mod dock_plugin;
use dock_plugin::{DockClosure, DockLocation, DockPlugin};

fn main() {
    let mut panel_builders = BTreeMap::new();

    let is_checked = Arc::new(Mutex::new(true));
    // Arc<Mutex<String>> allows edit, Arc<Mustex<&str>> does not
    let text_input = Arc::new(Mutex::new(String::from("Enter Text Here")));
    let slider_value = Arc::new(Mutex::new(100.0));


    #[derive(PartialEq)]
    enum Enum {
        First,
        Second,
        Third,
    }

    let enum_value = Arc::new(Mutex::new(Enum::First));

    let slider_value_clone_1 = Arc::clone(&slider_value);
    panel_builders.insert(
        DockLocation::Left,
        DockClosure::new(Arc::new(move |ui: &mut egui::Ui, asset_server| {
            ui.label("Left Corner of the Playground");
            let _image_handle: Handle<Image> = asset_server.load("icon_inverted.png");

            // checkbox
            ui.separator();
            let is_checked_clone = Arc::clone(&is_checked);
            let mut is_checked_lock = is_checked_clone
                .lock()
                .expect("is_checked : Failed to acquire lock");
            ui.checkbox(&mut *is_checked_lock, "Check Me Out in the Playground");

            // text intput
            ui.separator();
            let text_input_clone = Arc::clone(&text_input);
            let mut text_input_lock = text_input_clone.lock().unwrap();
            ui.text_edit_singleline(&mut *text_input_lock);

            // ui button
            ui.separator();
            if ui.button("Click Me").clicked() {
                *text_input_lock = String::from("Button Clicked");
            }

            // slider
            ui.separator();
            let slider_value_clone = Arc::clone(&slider_value_clone_1);
            let mut slider_value_lock = slider_value_clone.lock().unwrap();
            ui.add(egui::Slider::new(&mut *slider_value_lock, 0.001..=100.0).text("slider"));

            // radio button
            ui.separator();
            let enum_value_clone = Arc::clone(&enum_value);
            let mut enum_value_lock = enum_value_clone.lock().unwrap();
            ui.horizontal(|ui| {
                ui.radio_value(&mut *enum_value_lock, Enum::First, "First");
                ui.radio_value(&mut *enum_value_lock, Enum::Second, "Second");
                ui.radio_value(&mut *enum_value_lock, Enum::Third, "Third");
            });

            // collapsing header
            ui.separator();
            ui.collapsing("Click to See What is Hidden!", |ui| {
                ui.heading("Behold ALL the SECRETS...");
                ui.label(
                    egui::RichText::new("Not Much as it turns out!").color(egui::Color32::RED),
                );
                ui.separator();
                // @todo : what does speed do?
                ui.add(egui::DragValue::new(&mut *slider_value_lock).speed(0.01));
                ui.separator();
                // @audit : adding 5.0 to prevent full collapse and runtime error
                let text_edit = egui::TextEdit::singleline(&mut *text_input_lock)
                    .desired_width(*slider_value_lock + 5.0);
                ui.add(text_edit);
                ui.strong("Strong Text : Weak Secrets");
            });
        })),
    );

    panel_builders.insert(
        DockLocation::Right,
        DockClosure::new(Arc::new(|ui: &mut egui::Ui, asset_server| {
            ui.label("Right of the Playground");

            let _image_handle: Handle<Image> = asset_server.load("icon_inverted.png");
            // let texture_id = TextureId::from(image_handle);
            // ui.image(texture_id, [100.0, 100.0]);
            ui.separator();
            let is_collapsed = ui.collapsing("Scratchpad", |ui| {
                ui.code(
                    "if (secret) { 
                        hide(); 
                    } else {
                        show();
                    }",
                );
                ui.hyperlink("https://github.com/emilk/egui");
            });
            ui.separator();
            if is_collapsed.fully_closed() {
                ui.separator();
                ui.heading("Click to Expand");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Same");
                    ui.separator();
                    ui.label("uh");
                    ui.heading("WIDE LOAD LOL");
                    ui.label("no");
                    ui.separator();
                    ui.label("Row");
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label("Same");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("waah");
                        ui.separator();
                        ui.heading("CRY ME A RIVER!!! LOL ... NEED to WRAP OOF");
                        ui.separator();
                        ui.label("sniff");
                    });
                    ui.separator();
                    ui.label("Column");
                    ui.separator();
                });
            } else {
                let button = ui.button("Click to Toggle ScratchPad");
                if button.hovered() {
                    ui.separator();
                    ui.heading("Hovering");
                }
                if button.clicked() {
                    ui.separator();
                    // @todo : this just flashes on click, find a way to countdown
                    // before hiding
                    ui.label("Clicked");
                }
            }
        })),
    );
    panel_builders.insert(
        DockLocation::Top,
        DockClosure::new(Arc::new(|ui: &mut egui::Ui, asset_server| {
            ui.label("Top of the Playground");
            let _image_handle: Handle<Image> = asset_server.load("icon_inverted.png");
        })),
    );

    // gets moved to PanelType::Bottom ... @audit : Explain what's happening here lol
    let counter = Arc::new(Mutex::new(0));
    let slider_value_clone_2 = Arc::clone(&slider_value);
    panel_builders.insert(
        DockLocation::Bottom,
        DockClosure::new(Arc::new(move |ui: &mut egui::Ui, asset_server| {
            ui.label("Bottom of the Playground");

            let counter_clone = Arc::clone(&counter);
            let mut counter_lock = counter_clone.lock().unwrap();
            ui_counter_widget(&mut *ui, &mut *counter_lock);

            let _image_handle: Handle<Image> = asset_server.load("icon_inverted.png");
            // Create a "canvas" for drawing on that's 100% x 300 px
            let (response, _painter) =
                ui.allocate_painter(Vec2::new(ui.available_width(), 100.0), Sense::hover());
            // Get the relative position of our canvas
            let to_screen = RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );

            let slider_value_clone = Arc::clone(&slider_value_clone_2);
            let slider_value_lock = slider_value_clone.lock().unwrap();
            let y_offset = *slider_value_lock;

            // Create an absolute point
            let point = Pos2::ZERO;
            // Make the absolute point relative to the canvas container
            let point_in_screen = to_screen.transform_pos(point);
            // e.g. x: 330.0, y: 245.0
            ui.allocate_space(Vec2::new(0.0, 20.0 + y_offset));
            ui.label(format!("{:?}", point_in_screen));

            // Absolutely place Ui relative to the container
            let position = Pos2::new(10.0, 10.0 + y_offset);

            let _animation_clip_button = ui.put(
                Rect {
                    min: to_screen.transform_pos(position),
                    max: to_screen.transform_pos(Pos2 {
                        x: position.x + 150.0,
                        y: position.y + 100.0,
                    }),
                },
                egui::Button::new("Animation Clip"),
            );
        })),
    );
    let dock_plugin = DockPlugin::new(panel_builders);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(dock_plugin)
        .run();
}

fn ui_counter_widget(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the  buttons and label on the same row
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
