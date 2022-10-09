use bevy::prelude::*;
mod controls;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(controls::controller_system::gamepad_connections)
        .run();
}
