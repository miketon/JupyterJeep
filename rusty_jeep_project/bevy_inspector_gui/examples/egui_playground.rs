use bevy::prelude::*;

mod dock_plugin;
use dock_plugin::DockPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DockPlugin)
        .run();
}
