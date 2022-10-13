use crate::globals::character_modifiers::{Behaviors, Enemy, Player};
use crate::globals::scene_modifiers::EnemyTimer;
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_enemies(
    mut commands: Commands,
    server: Res<AssetServer>,
    enemies: Query<Entity, With<Enemy>>,
    player_query: Query<&Player>,
    mut enemy_timer: ResMut<EnemyTimer>,
    time: Res<Time>,
) {
    const DISTANCE: f32 = 50.0;
    if !player_query.is_empty() {
        if enemies.iter().len() < 256 {
            enemy_timer.timer.tick(time.delta());
            if enemy_timer.timer.finished() {
                let player = player_query.iter().next().unwrap();
                let zombie_sprite: Handle<Image> = server.load("images/Zombie.png");
                for i in 0..8 {
                    let mut rng = rand::thread_rng();
                    let x_off: f32 = rng.gen::<f32>() * DISTANCE;
                    let y_off: f32 = rng.gen::<f32>() * DISTANCE;
                    let quadrant_offset: Vec2 = match rng.gen_range(0..20) {
                        0 => Vec2::new(300.0, 15.0),
                        1 => Vec2::new(300.0, 52.0),
                        2 => Vec2::new(300.0, 90.0),
                        3 => Vec2::new(75.0, 126.0),
                        4 => Vec2::new(15.0, 163.0),
                        5 => Vec2::new(-15.0, 163.0),
                        6 => Vec2::new(-75.0, 126.0),
                        7 => Vec2::new(-300.0, 90.0),
                        8 => Vec2::new(-300.0, 52.0),
                        9 => Vec2::new(-300.0, 15.0),
                        10 => Vec2::new(-300.0, -15.0),
                        11 => Vec2::new(-300.0, -52.0),
                        12 => Vec2::new(-300.0, -90.0),
                        13 => Vec2::new(-75.0, -126.0),
                        14 => Vec2::new(-15.0, -163.0),
                        15 => Vec2::new(15.0, -163.0),
                        16 => Vec2::new(75.0, -126.0),
                        17 => Vec2::new(300.0, -90.0),
                        18 => Vec2::new(300.0, -52.0),
                        19 => Vec2::new(300.0, -15.0),
                        _default => Vec2::new(300.0, -15.0),
                    };
                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: zombie_sprite.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    x_off + quadrant_offset.x,
                                    y_off + quadrant_offset.y,
                                    player.location.z,
                                ),
                                scale: Vec3::new(0.5, 0.5, 0.5),
                                rotation: Quat::from_rotation_z(360.0),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Enemy {
                            max_speed: 20.0,
                            acceleration: 40.0,
                            current_speed: Vec3::splat(0.0),
                            x_direction: false,
                            y_direction: false,
                            behavior: Behaviors::Smooth,
                        });
                }
            }
        }
    }
}
