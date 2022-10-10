use bevy::{prelude::*, window::WindowMode};

mod controls;
mod setup;
mod states;

fn main() {
    App::new()
        /*         .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: String::from("Who Cares"),
            mode: WindowMode::Windowed,
            resizable: false,
            ..default()
        }) */
        .add_plugins(DefaultPlugins)
        .add_system(controls::controller_system::gamepad_connections)
        //.add_system(setup)
        .add_startup_system(setup::menu::ui_camera)
        .add_startup_system(setup::menu::main_menu)
        .add_state(states::base::GameState::MainMenu)
        .run();
}

/* fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
 */
