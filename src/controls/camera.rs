use crate::setup::camera::MyCamera;
use bevy::prelude::*;

pub fn move_camera(
    mut camera: Query<(&mut Transform, &mut MyCamera, With<MyCamera>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if keys.pressed(KeyCode::Minus) {
        for (mut transform, _cam, _search) in &mut camera {
            transform.scale += 5.0 * time.delta_seconds();
            println!("Camera scale: {:?}", transform.scale);
        }
    }
    if keys.pressed(KeyCode::Equals) {
        for (mut transform, _cam, _search) in &mut camera {
            transform.scale -= 5.0 * time.delta_seconds();
            println!("Camera scale: {:?}", transform.scale);
        }
    }
}
