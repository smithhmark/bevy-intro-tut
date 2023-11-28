use bevy::prelude::*;

fn main() {
    App::new().add_systems(Update, hello_world_system).run();
    println!("Main says, 'Hello, world!' as it exits");
}

fn hello_world_system() {
    println!("a system says, 'Hello, world!'");
}
