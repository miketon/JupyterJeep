use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
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

/*
Entities and Components are great for representing complex, query-able groups of
data. But most Apps will also require "globally unique" data of some kind.
In Bevy ECS, we represent globally unique data using Resources.

Here are some examples of data that could be encoded as Resources:
- Elapsed Time
- Asset Collections (sounds, textures, meshes)
- Renderers
 */

// Resources are just types that implement the Resource trait
// Using time to rate limit greeting people
#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // Update our timer with the time elapsed since the last update
    // If that caused the timer to finish, we can greet people
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
}

// Moving our systems into a plugin
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add a repeating timer resource to our plugin
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

// https://bevy-cheatbook.github.io/