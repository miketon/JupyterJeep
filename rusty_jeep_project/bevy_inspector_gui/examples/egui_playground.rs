use bevy::prelude::*;
use bevy_inspector_egui::egui;
use std::collections::HashMap;
use std::sync::Arc;

mod dock_plugin;
use dock_plugin::DockPlugin;

fn main() {
    let mut panel_builders = HashMap::new();
    panel_builders.insert(
        "left".to_string(),
        Arc::new(|ui: &mut egui::Ui| {
            ui.label("Left Corner of the Playground");
            let mut is_checked = true;
            ui.checkbox(&mut is_checked, "Check Me Out in the Playground");
        }) as Arc<dyn Fn(&mut egui::Ui) + Send + Sync + 'static>,
    );
    let dock_plugin = DockPlugin::new(panel_builders);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(dock_plugin)
        .run();
}
