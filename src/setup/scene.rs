use crate::controls::camera::move_camera;
use crate::controls::player::move_character;
use crate::globals::character_modifiers::*;
use crate::globals::scene_modifiers::*;
use crate::globals::ui_modifiers::{GameUi, Inventory, UiGrid};
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

pub fn initialize_game(
    mut commands: Commands,
    server: Res<AssetServer>,
    players: Query<Entity, With<Player>>,
    home_grid: Query<Entity, &FarmLand>,
    background_grid: Query<&Background>,
    game_ui: Query<&GameUi>,
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
    if game_ui.is_empty() {
        let inventory_slot = server.load("images/InventorySlot.png");
        let daylight: Handle<Image> = server.load("images/Day.png");
        let start: Handle<Image> = server.load("images/1200.png");
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                color: UiColor(Color::rgba(0., 0., 0., 0.)),
                ..default()
            })
            .insert(GameUi)
            .insert(Name::new("Ui Parent"))
            .add_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(100.0),
                                height: Val::Percent(10.0),
                            },
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexStart,

                            ..default()
                        },
                        color: UiColor(Color::rgba(0., 0., 0., 0.)),
                        ..default()
                    })
                    .insert(Name::new("Time Row"))
                    .with_children(|timer_row| {
                        timer_row
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Percent(20.0),
                                        height: Val::Percent(100.0),
                                    },
                                    flex_direction: FlexDirection::Row,
                                    padding: UiRect {
                                        left: Val::Px(8.0),
                                        top: Val::Px(8.0),
                                        right: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                    },
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,

                                    ..default()
                                },
                                color: UiColor(Color::rgba(0., 0., 0., 0.)),
                                ..default()
                            })
                            .insert(Name::new("Top Left Block"))
                            .with_children(|time_block| {
                                time_block.spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size {
                                            width: Val::Percent(50.0),
                                            height: Val::Percent(100.0),
                                        },
                                        margin: UiRect {
                                            right: Val::Px(8.0),
                                            left: Val::Px(0.0),
                                            bottom: Val::Px(0.0),
                                            top: Val::Px(0.0),
                                        },
                                        ..default()
                                    },
                                    image: UiImage(start.clone()),
                                    ..default()
                                });
                                time_block.spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size {
                                            width: Val::Percent(45.0),
                                            height: Val::Percent(100.0),
                                        },
                                        ..default()
                                    },
                                    image: UiImage(daylight.clone()),
                                    ..default()
                                });
                            });
                    });
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            size: Size {
                                width: Val::Percent(100.0),
                                height: Val::Percent(10.0),
                                ..default()
                            },
                            ..default()
                        },
                        color: UiColor(Color::rgba(0., 0., 0., 0.)),
                        ..default()
                    })
                    .insert(Name::new("Inventory Row"))
                    .with_children(|invent_row| {
                        invent_row
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::SpaceEvenly,
                                    flex_direction: FlexDirection::Row,
                                    size: Size {
                                        width: Val::Percent(60.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                color: UiColor(Color::rgba(0., 0., 0., 0.)),
                                ..default()
                            })
                            .insert(Name::new("Inventory Block"))
                            .insert(Inventory)
                            .with_children(|invent_block| {
                                for i in 1..=6 {
                                    spawn_inventory_slot(invent_block, inventory_slot.clone(), i);
                                }
                            });
                    });
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
