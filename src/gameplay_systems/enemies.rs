use crate::globals::character_modifiers::{
    Behaviors, Bursting, Charging, Cooldown, Enemy, EnemyData, Player,
};
use crate::globals::scene_modifiers::EnemyTimer;
use crate::states::GameState;
use bevy::prelude::*;
use libm::{atan2f, cosf, sinf};
use rand::Rng;
use std::f32::consts::PI;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(spawn_enemies))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_bursters))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_enemies));
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    server: Res<AssetServer>,
    enemies: Query<Entity, With<Enemy>>,
    player_query: Query<&Player>,
    mut enemy_timer: ResMut<EnemyTimer>,
    time: Res<Time>,
    enemy: Res<EnemyData>,
) {
    const DISTANCE: f32 = 50.0;
    if !player_query.is_empty() {
        let enemy_len = enemies.iter().len() as i32;
        if enemy_len <= 256 {
            enemy_timer.timer.tick(time.delta());
            if enemy_timer.timer.finished() {
                let zombie_sprite: Handle<Image> = server.load("images/Zombie.png");
                let mosquito_sprite: Handle<Image> = server.load("images/Mosquito.png");
                let crab_sprite: Handle<Image> = server.load("images/Crab.png");
                let fox_sprite: Handle<Image> = server.load("images/Fox.png");
                let wolf_sprite: Handle<Image> = server.load("images/Wolf.png");
                for i in 1..=10 {
                    let mut name_string = String::from("Enemy ");
                    name_string += &(enemy_len + i).to_string();
                    let mut rng = rand::thread_rng();
                    let x_off: f32 = rng.gen::<f32>() * DISTANCE;
                    let y_off: f32 = rng.gen::<f32>() * DISTANCE;
                    let sprite = match rng.gen_range(0..=4) {
                        0 => zombie_sprite.clone(),
                        1 => mosquito_sprite.clone(),
                        2 => crab_sprite.clone(),
                        3 => fox_sprite.clone(),
                        4 => wolf_sprite.clone(),
                        _default => wolf_sprite.clone(),
                    };

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
                            texture: sprite,
                            transform: Transform {
                                translation: Vec3::new(
                                    x_off + quadrant_offset.x,
                                    y_off + quadrant_offset.y,
                                    1.0,
                                ),
                                scale: Vec3::new(0.5, 0.5, 0.5),
                                rotation: Quat::from_rotation_z(PI),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Enemy {
                            max_speed: enemy.max_speed,
                            acceleration: enemy.acceleration,
                            behavior: enemy.behavior,
                            charge_delay: enemy.charge_delay,
                            cooldown: enemy.cooldown,
                            ..default()
                        })
                        .insert(Name::new(name_string));
                }
            }
        }
    }
}

pub fn move_bursters(
    mut commands: Commands,
    mut bursters: Query<(
        &mut Transform,
        &mut Enemy,
        Entity,
        &Bursting,
        With<Enemy>,
        With<Bursting>,
    )>,
    time: Res<Time>,
) {
    bursters.iter_mut().for_each(
        |(mut transform, mut stats, entity, burst, _search1, _search2)| {
            if stats.current_speed <= 0.0 {
                commands.entity(entity).remove::<Bursting>();
            }
            transform.translation += Vec3::new(
                -sinf(burst.angle) * stats.current_speed * time.delta_seconds(),
                cosf(burst.angle) * stats.current_speed * time.delta_seconds(),
                0.0,
            );
            stats.current_speed -= stats.acceleration * time.delta_seconds() / 2.0;
        },
    );
}

pub fn move_enemies(
    mut commands: Commands,
    players: Query<&Player>,
    mut enemies: Query<(
        &mut Transform,
        &mut Enemy,
        Entity,
        With<Enemy>,
        Without<Bursting>,
        Without<Cooldown>,
    )>,
    time: Res<Time>,
) {
    if !players.is_empty() {
        let player = players.iter().next().unwrap();
        let target = player.location;
        enemies.iter_mut().for_each(
            |(mut transform, mut stats, entity, _search1, _search2, _search3)| {
                let dx = target.x - transform.translation.x;
                let dy = target.y - transform.translation.y;
                let angle = atan2f(dy, dx) - PI / 2.0;
                let new_quat = Quat::from_rotation_z(angle);
                match stats.behavior {
                    Behaviors::Smooth => {
                        if new_quat.abs_diff_eq(transform.rotation, 0.005) {
                            if stats.current_speed < stats.max_speed {
                                stats.current_speed += stats.acceleration * time.delta_seconds();
                            }
                        } else {
                            if stats.current_speed > stats.max_speed / 2.0 {
                                stats.current_speed -=
                                    stats.acceleration * time.delta_seconds() / 5.0;
                            }
                        }
                        transform.rotation = new_quat;
                        transform.translation += Vec3::new(
                            -sinf(angle) * stats.current_speed * time.delta_seconds(),
                            cosf(angle) * stats.current_speed * time.delta_seconds(),
                            0.0,
                        );
                    }
                    Behaviors::Burst => {
                        if stats.current_speed < stats.max_speed {
                            stats.current_speed += stats.acceleration * time.delta_seconds();
                        } else {
                            commands.entity(entity).insert(Bursting { angle });
                        }
                        transform.rotation = new_quat;
                    }
                    Behaviors::Charge => {
                        if dx.abs() + dy.abs() <= stats.max_speed {
                            commands
                                .entity(entity)
                                .insert(Cooldown {
                                    timer: Timer::from_seconds(20.0, false),
                                })
                                .insert(Charging {
                                    angle,
                                    charge_delay: Timer::from_seconds(1.0, false),
                                });
                        }

                        if stats.current_speed < stats.max_speed {
                            stats.current_speed += stats.acceleration * time.delta_seconds();
                        }
                        transform.rotation = new_quat;
                        transform.translation += Vec3::new(
                            -sinf(angle) * stats.current_speed * time.delta_seconds(),
                            cosf(angle) * stats.current_speed * time.delta_seconds(),
                            0.0,
                        );
                    }
                }
            },
        );
    }
}
