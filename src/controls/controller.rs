use bevy::prelude::*;

pub struct MyGamepad(Gamepad);

pub fn gamepad_connections(
    mut commands: Commands,
    the_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
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
                _default => println!("Hit {:?} with val {:?}", kind, val),
            },
        }
    }
}
