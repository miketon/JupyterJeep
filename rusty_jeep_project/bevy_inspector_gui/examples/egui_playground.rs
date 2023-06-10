use bevy::prelude::*;
use bevy_inspector_egui::egui;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod dock_plugin;
use dock_plugin::{DockPlugin, PanelData, PanelType};

fn main() {
    let mut panel_builders = HashMap::new();

    let is_checked = Arc::new(Mutex::new(true));
    // Arc<Mutex<String>> allows edit, Arc<Mustex<&str>> does not
    let text_input = Arc::new(Mutex::new(String::from("Enter Text Here")));

    panel_builders.insert(
        PanelType::Left,
        PanelData::new(Arc::new(move |ui: &mut egui::Ui| {
            ui.label("Left Corner of the Playground");

            // checkbox
            let is_checked_clone = Arc::clone(&is_checked);
            let mut is_checked_lock = is_checked_clone
                .lock()
                .expect("is_checked : Failed to acquire lock");
            ui.checkbox(&mut *is_checked_lock, "Check Me Out in the Playground");

            // text intput
            let text_input_clone = Arc::clone(&text_input);
            let mut text_input_lock = text_input_clone.lock().unwrap();
            ui.text_edit_singleline(&mut *text_input_lock);

            let width = 300.0;
            let text_edit = egui::TextEdit::singleline(&mut *text_input_lock).desired_width(width);
            ui.add(text_edit);
        })),
    );
    panel_builders.insert(
        PanelType::Right,
        PanelData::new(Arc::new(|ui: &mut egui::Ui| {
            ui.label("Right of the Playground");
        })),
    );
    panel_builders.insert(
        PanelType::Top,
        PanelData::new(Arc::new(|ui: &mut egui::Ui| {
            ui.label("Top of the Playground");
        })),
    );
    panel_builders.insert(
        PanelType::Bottom,
        PanelData::new(Arc::new(|ui: &mut egui::Ui| {
            ui.label("Bottom of the Playground");
        })),
    );
    let dock_plugin = DockPlugin::new(panel_builders);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(dock_plugin)
        .run();
}
