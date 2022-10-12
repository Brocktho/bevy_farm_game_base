use crate::controls::camera::move_camera;
use crate::controls::player::move_character;
use crate::gameplay_systems::enemies::spawn_enemies;
use crate::globals::character_modifiers::*;
use crate::globals::scene_modifiers::*;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use rand::Rng;
use std::time::Duration;
pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameLoop).with_system(initialize_game))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_character))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_camera))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_enemies))
            .add_system_set(
                SystemSet::on_update(GameState::GameLoop)
                    .after(initialize_game)
                    .with_system(spawn_enemies),
            );
    }
}

pub fn find_action_square(
    mut active_square: Query<(&mut Handle<Image>, With<GameGrid>)>,
    mut squares: Query<(&Transform, &mut Handle<Image>, With<GameGrid>)>,
    player: Query<&Player>,
    asset_server: Res<AssetServer>,
) {
    if !player.is_empty() {
        let focus_tile: Handle<Image> = asset_server.load("images/Highlight.png");
        let plain_tile: Handle<Image> = asset_server.load("images/Dirt.png");
        let player_found = player.iter().next().unwrap();
        let player_pos = player_found.location;
    }
}

pub fn generate_timers(
    mut commands: Commands,
    cycle_timer: Query<&CycleTimer>,
    day_timer: Query<&DayTimer>,
    night_timer: Query<&NightTimer>,
    enemy_timer: Query<&EnemyTimer>,
) {
    if cycle_timer.is_empty() {
        commands.insert_resource(CycleTimer {
            timer: Timer::from_seconds(120.0, true),
        });
    }
}

pub fn move_enemies(
    players: Query<&Player>,
    mut enemies: Query<(&mut Transform, &mut Enemy, With<Enemy>)>,
    time: Res<Time>,
) {
    if !players.is_empty() {
        let player = players.iter().next().unwrap();
        let target = player.location;
        enemies
            .iter_mut()
            .for_each(|(mut transform, mut stats, _search)| {
                let dx = target.x - transform.translation.x;
                let dy = target.y - transform.translation.y;
                if dx > 0.0 {
                    if !stats.x_direction {
                        stats.current_speed.x = 0.0;
                    }
                    if stats.current_speed.x < stats.max_speed {
                        stats.current_speed.x += stats.acceleration * time.delta_seconds();
                    }
                    transform.translation.x += stats.current_speed.x * time.delta_seconds();
                    stats.x_direction = true;
                } else if dx < 0.0 {
                    if stats.x_direction {
                        stats.current_speed.x = 0.0;
                    }
                    if stats.current_speed.x < stats.max_speed {
                        stats.current_speed.x += stats.acceleration * time.delta_seconds();
                    }
                    transform.translation.x -= stats.current_speed.x * time.delta_seconds();
                    stats.x_direction = false;
                }

                if dy > 0.0 {
                    if !stats.y_direction {
                        stats.current_speed.y = 0.0;
                    }
                    if stats.current_speed.y < stats.max_speed {
                        stats.current_speed.y += stats.acceleration * time.delta_seconds();
                    }
                    transform.translation.y += stats.current_speed.y * time.delta_seconds();
                    stats.y_direction = true;
                } else if dy < 0.0 {
                    if stats.y_direction {
                        stats.current_speed.y = 0.0;
                    }
                    if stats.current_speed.y < stats.max_speed {
                        stats.current_speed.y += stats.acceleration * time.delta_seconds();
                    }
                    transform.translation.y -= stats.current_speed.y * time.delta_seconds();
                    stats.y_direction = false;
                }
            });
    }
}

pub fn initialize_game(
    mut commands: Commands,
    server: Res<AssetServer>,
    players: Query<Entity, With<Player>>,
    home_grid: Query<Entity, &FarmLand>,
    background_grid: Query<&Background>,
) {
    let handle: Handle<Image> = server.load("images/Bunny.png");
    if players.is_empty() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: handle.clone(),
                transform: Transform {
                    translation: Vec3::splat(1.0),
                    scale: Vec3::splat(0.5),
                    //translation:
                    ..default()
                },
                ..default()
            })
            .insert(Player {
                //health: 3,
                ..default()
            })
            .insert(Name::new("Player"));
        commands.insert_resource(UiImage(handle));
        commands.insert_resource(EnemyTimer {
            timer: Timer::new(Duration::from_secs(3), true),
        });
    }
    if home_grid.is_empty() {
        let base = server.load("images/Dirt.png");
        commands
            .spawn_bundle(SpatialBundle { ..default() })
            .insert(FarmLand)
            .insert(Name::new("FarmParent"))
            .with_children(|parent| {
                for i in 0..16 {
                    for j in 0..16 {
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: base.clone(),
                                transform: Transform {
                                    translation: Vec3::new(
                                        14.0 * j as f32 - 105.0,
                                        14.0 * i as f32 - 105.0,
                                        0.02,
                                    ),
                                    scale: Vec3::splat(0.3),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(GameGrid { x: j, y: i });
                    }
                }
            });
    }
    if background_grid.is_empty() {
        let grass1 = server.load("images/GrassTile.png");
        let grass2 = server.load("images/GrassTile2.png");
        let grass3 = server.load("images/GrassTile3.png");
        let mut rand = rand::thread_rng();
        commands
            .spawn_bundle(SpatialBundle { ..default() })
            .insert(Background)
            .insert(Name::new("BackgroundParent"))
            .with_children(|parent| {
                for i in 0..30 {
                    for j in 0..30 {
                        let val = rand.gen_range(0..2);
                        parent.spawn_bundle(SpriteBundle {
                            texture: if val == 2 {
                                grass1.clone()
                            } else if val == 1 {
                                grass2.clone()
                            } else {
                                grass3.clone()
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    28.0 * j as f32 - 280.0,
                                    28.0 * i as f32 - 280.0,
                                    0.01,
                                ),
                                ..default()
                            },
                            ..default()
                        });
                    }
                }
            });
    }
}
