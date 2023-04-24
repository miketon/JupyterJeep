use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}

fn hello_world(){
    println!("Hello world! Bevy birds are NOT angry!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands:Commands){
    commands.spawn((Person, Name("Mario".to_string())));
    commands.spawn((Person, Name("Luigi".to_string())));
    commands.spawn((Person, Name("Peach".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query{
        println!("Hello {}!", name.0);
    }
}