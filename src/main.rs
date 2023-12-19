use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (greet))
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("MumboJumboYouAreAFK".to_string())));
    commands.spawn((Person, Name("Grianne".to_string())));
    commands.spawn((Person, Name("AlexanderHamilton7298".to_string())));
}

fn greet(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello there, {}!", name.0);
    }
}
