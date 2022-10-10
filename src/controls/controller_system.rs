use bevy::prelude::*;

pub struct MyGamepad(Gamepad);

pub struct Bullet {
    x: f32, // all in m
    y: f32,
    z: f32,
    velocity: Velocity,     // dx/dt m/s
    acceleration: Velocity, //
}

pub struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Acceleration {
    x: f32,
    y: f32,
    z: f32,
}

pub fn gamepad_connections(
    mut commands: Commands,
    the_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in gamepad_evr.iter() {
        let id = ev.gamepad;
        match ev.event_type {
            GamepadEventType::Connected => {
                println!("New gamepad {:?}", id);

                if the_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Disconnected {:?}", id);

                if let Some(MyGamepad(old_id)) = the_gamepad.as_deref() {
                    if *old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            GamepadEventType::AxisChanged(kind, val) => {
                println!(
                    "gamepad: {:?} changed direction type {:?} with val {:?}",
                    id, kind, val
                );
            }

            GamepadEventType::ButtonChanged(kind, val) => match kind {
                GamepadButtonType::RightTrigger => create_projectile(commands, meshes, materials),
                _default => println!("Hit {:?} with val {:?}", kind, val),
            },
        }
    }
}

pub fn create_projectile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { ..default() })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..default()
    });
}
