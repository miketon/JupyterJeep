use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn hello_world() {
    println!("Hello world! Bevy birds are NOT angry!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    println!("Adding people...");
    commands.spawn((Person, Name("Mario".to_string())));
    commands.spawn((Person, Name("Luigi".to_string())));
    commands.spawn((Person, Name("Peach".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

// Moving our systems into a plugin
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_startup_system(hello_world)
            .add_system(greet_people); // why does this not print as add_startup_system?
    }
}
