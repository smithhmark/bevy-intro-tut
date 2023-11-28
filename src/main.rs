use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(PreStartup, setup)
        .add_systems(Update, say_hello)
        .add_systems(Update, hello_world_system)
        .run();
    println!("Main says, 'Hello, world!' as it exits");
}

fn hello_world_system() {
    println!("a system says, 'Hello, world!'");
}

fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Mark".to_string(),
    });
    commands.spawn(Person {
        name: "Oscar".to_string(),
    });
}

fn say_hello(query: Query<&Person>) {
    for per in &query {
        println!("Hello {}!", per.name);
    }
}

#[derive(Component)]
struct Person {
    pub name: String,
}
