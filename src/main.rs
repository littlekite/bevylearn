use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_startup_system(step);
    app.run();
}

fn step(){
    println!("hello world")
}