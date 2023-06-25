use bevy::prelude::*;

const MAX_MESSAGES: usize = 50;
const DEBUG_TEXT_COLOR: Color = Color::WHITE;

#[derive(Resource)]
struct DebugText {
    messages: Vec<String>,
}

impl DebugText {
    fn add_debug_message(&mut self, message: String) {
        self.messages.push(message);
        // Limit the number of messages displayed
        if self.messages.len() > MAX_MESSAGES {
            self.messages.remove(0);
        }
    }
}

pub struct DebugTextPlugin;

impl DebugTextPlugin {
    pub fn new() -> Self {
        DebugTextPlugin
    }
}

/// Implementing the Plugin trait for DebugTextPlugin
/// - `build` method : called on app.add_plugin(DebugTextPlugin)
impl Plugin for DebugTextPlugin {
    /// Adds the DebugText resource to the app
    /// - needs to mutably borrow the app ...
    /// @todo : Understand what is getting mutated in the app?
    /// ANSWER ... aw we are inserting a resource, we are mutating the app
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugText {
            messages: Vec::new(),
        })
        .add_startup_system(setup)
        .add_system(update_debug_text)
        .add_system(some_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load the font
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // create a ui text component
    commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Heyoooooo!".to_string(),
                style: TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: DEBUG_TEXT_COLOR,
                },
            }],
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_debug_text(mut query: Query<&mut Text>, debug_text: Res<DebugText>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = "What's up?".to_string();
        /*
        debug_text
            .messages
            .iter()
            .map(|message| format!("{}\n", message))
            .collect();
        */
    }
}

fn some_system(mut debug_text: ResMut<DebugText>) {
    debug_text.add_debug_message("Hello Yall".to_string());
}
