use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(PreStartup, setup)
        .add_systems(Update, say_hello)
        .add_systems(Update, say_job)
        .add_systems(Update, people_with_jobs)
        .add_systems(Update, people_without_jobs)
        .add_systems(Update, jobs)
        .add_systems(Update, hello_world_system)
        .run();
    println!("Main says, 'Hello, world!' as it exits");
}

fn hello_world_system() {
    println!("a system says, 'Hello, world!'");
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Mark".to_string(),
        },
        Employed { job: Job::Nerd },
    ));
    commands.spawn((
        Person {
            name: "Oscar".to_string(),
        },
        Employed { job: Job::HousePet },
    ));
    commands.spawn(Person {
        name: "Judy".to_string(),
    });
}

fn say_hello(query: Query<&Person>) {
    for per in &query {
        println!("Hello {}!", per.name);
    }
}

fn people_with_jobs(query: Query<&Person, With<Employed>>) {
    for per in &query {
        println!("Hello {}, you have a job", per.name);
    }
}

fn people_without_jobs(query: Query<&Person, Without<Employed>>) {
    for per in &query {
        println!("Hello {}, you don't have a job", per.name);
    }
}

fn jobs(query: Query<(&Person, &Employed)>) {
    for (per, emp) in &query {
        println!("{} is a {:?}", per.name, emp.job);
    }
}

fn say_job(query: Query<&Employed>) {
    for emp in &query {
        println!("this person is a {:?}!", emp.job);
    }
}

#[derive(Component)]
struct Person {
    pub name: String,
}

#[derive(Debug)]
enum Job {
    Nerd,
    //Docter,
    //Teacher,
    HousePet,
}

#[derive(Component)]
struct Employed {
    pub job: Job,
}
