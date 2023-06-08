use bevy::prelude::*;
use bevy_inspector_egui::egui;

mod dock_plugin;
use dock_plugin::DockPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DockPlugin::new(|ui: &mut egui::Ui| {
            ui.label("Right Corner of the Playground");
            let mut is_checked = true;
            ui.checkbox(&mut is_checked, "Check Me Out in the Playground");
        }))
        .run();
}
