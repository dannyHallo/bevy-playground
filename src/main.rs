use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Update, hello_world_system);
    app.run();
}

fn hello_world_system() {
    println!("Hello World!");
}
