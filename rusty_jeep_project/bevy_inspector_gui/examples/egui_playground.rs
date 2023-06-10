use bevy::prelude::*;
use bevy_inspector_egui::egui;
use std::collections::HashMap;
use std::sync::Arc;

mod dock_plugin;
use dock_plugin::{DockPlugin, PanelData, PanelType};

fn main() {
    let mut panel_builders = HashMap::new();
    panel_builders.insert(
        PanelType::Left,
        PanelData::new(Arc::new(|ui: &mut egui::Ui| {
            ui.label("Left Corner of the Playground");
            let mut is_checked = true;
            ui.checkbox(&mut is_checked, "Check Me Out in the Playground");
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
