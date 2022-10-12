use crate::globals::character_modifiers::Player;
use crate::states::GameState;
use bevy::prelude::*;

pub fn move_character(
    mut _commands: Commands,
    mut character: Query<(&mut Transform, &mut Player, With<Player>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keys.pressed(KeyCode::Z) {
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.z {
                player.current_speed.z += player.acceleration * time.delta_seconds();
            }
            transform.translation.z += player.current_speed.z * time.delta_seconds();
            player.location = transform.translation;
            println!("Z index: {:?}", transform);
        }
    }
    if keys.pressed(KeyCode::X) {
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.z {
                player.current_speed.z += player.acceleration * time.delta_seconds();
            }
            transform.translation.z -= player.current_speed.z * time.delta_seconds();
            player.location = transform.translation;
            println!("Z index: {:?}", transform);
        }
    }
    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
        // up
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.y {
                player.current_speed.y += player.acceleration * time.delta_seconds();
            }
            transform.translation.y += player.current_speed.y * time.delta_seconds();
            player.location = transform.translation;
        }
    }
    if keys.just_released(KeyCode::W) || keys.just_released(KeyCode::Up) {
        for (mut _transform, mut player, _with) in &mut character {
            player.current_speed.y = 0.0;
        }
    }
    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
        // left
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.x {
                player.current_speed.x += player.acceleration * time.delta_seconds();
            }
            transform.translation.x -= player.current_speed.x * time.delta_seconds();
            player.location = transform.translation;
        }
    }
    if keys.just_released(KeyCode::A) || keys.just_released(KeyCode::Left) {
        for (mut _transform, mut player, _with) in &mut character {
            player.current_speed.x = 0.0;
        }
    }
    if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
        // down
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.y {
                player.current_speed.y += player.acceleration * time.delta_seconds();
            }
            transform.translation.y -= player.current_speed.y * time.delta_seconds();
            player.location = transform.translation;
        }
    }
    if keys.just_released(KeyCode::S) || keys.just_released(KeyCode::Down) {
        for (mut _transform, mut player, _with) in &mut character {
            player.current_speed.y = 0.0;
        }
    }
    if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
        // right
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.x {
                player.current_speed.x += player.acceleration * time.delta_seconds();
            }
            transform.translation.x += player.current_speed.x * time.delta_seconds();
            player.location = transform.translation;
        }
    }
    if keys.just_released(KeyCode::D) || keys.just_released(KeyCode::Right) {
        for (mut _transform, mut player, _with) in &mut character {
            player.current_speed.x = 0.0;
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        game_state
            .push(GameState::Settings)
            .map_err(|err| error!("Unable to open Settings: {}", err))
            .unwrap();
    }
}
