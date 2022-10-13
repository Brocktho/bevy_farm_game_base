use crate::controls::camera::move_camera;
use crate::controls::player::move_character;
use crate::gameplay_systems::enemies::spawn_enemies;
use crate::globals::character_modifiers::*;
use crate::globals::scene_modifiers::*;
use crate::globals::ui_modifiers::Inventory;
use crate::globals::ui_modifiers::UiGrid;
use crate::states::GameState;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;
pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameLoop).with_system(initialize_game))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_character))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_camera));
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

pub fn initialize_game(
    mut commands: Commands,
    server: Res<AssetServer>,
    players: Query<Entity, With<Player>>,
    home_grid: Query<Entity, &FarmLand>,
    background_grid: Query<&Background>,
    inventory_grid: Query<&UiGrid>,
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
                        let mut name_string = String::from("Farm Grid ");
                        name_string += &(i.to_string());
                        name_string += " ";
                        name_string += &(j.to_string());
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
                            .insert(GameGrid { x: j, y: i })
                            .insert(Name::new(name_string));
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
                        let mut name_string = String::from("Background ");
                        name_string += &(i.to_string());
                        name_string += " ";
                        name_string += &(j.to_string());
                        parent
                            .spawn_bundle(SpriteBundle {
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
                            })
                            .insert(Name::new(name_string));
                    }
                }
            });
    }
    if inventory_grid.is_empty() {
        let inventory_slot = server.load("images/InventorySlot.png");
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,

                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(7.5),
                        ..default()
                    },

                    ..default()
                },
                color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            })
            .insert(Inventory)
            .insert(Name::new("Inventory Parent"))
            .add_children(|parent| {
                for i in 1..=6 {
                    spawn_inventory_slot(parent, inventory_slot.clone(), i)
                }
            });
    }
}

pub fn spawn_inventory_slot(parent: &mut ChildBuilder, texture: Handle<Image>, grid_x: i32) {
    let mut name_string = String::from("Inventory ");
    name_string += &grid_x.to_string();
    parent
        .spawn_bundle(ImageBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(9.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                aspect_ratio: Some(1.0),
                ..default()
            },
            image: UiImage(texture),
            ..default()
        })
        .insert(UiGrid { x: grid_x, y: 0 })
        .insert(Name::new(name_string));
}
