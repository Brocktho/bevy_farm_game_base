use bevy::prelude::{Camera2dBundle, *};
use bevy::render::camera::ScalingMode;

#[derive(Component)]
pub struct MyCamera;

pub fn ui_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 250.0,
                scaling_mode: ScalingMode::FixedVertical(1.),
                ..default()
            },
            ..default()
        })
        .insert(MyCamera);
}
