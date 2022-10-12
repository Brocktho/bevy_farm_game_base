use crate::setup::menu::{Grid, KeyFocus, RevertScale, Scale};
use crate::states::base::GameState;
use bevy::app::AppExit;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

pub struct SettingsPlugin;

#[derive(Component, Clone, Copy)]
pub enum SettingsItem {
    Resume,
    MainMenu,
    Quit,
}

#[derive(Component)]
pub struct SettingsMenu;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Settings).with_system(init_settings))
            .add_system_set(SystemSet::on_update(GameState::Settings).with_system(hover_menu))
            .add_system_set(SystemSet::on_update(GameState::Settings).with_system(remove_hover))
            .add_system_set(
                SystemSet::on_update(GameState::Settings).with_system(handle_settings_interactions),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Settings).with_system(navigate_settings_with_keys),
            )
            .add_system_set(SystemSet::on_pause(GameState::Settings).with_system(remove_settings))
            .add_system_set(SystemSet::on_exit(GameState::Settings).with_system(remove_settings));
    }
}

pub fn remove_settings(mut commands: Commands, query: Query<Entity, With<SettingsMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn hover_menu(mut items: Query<&mut Transform, With<Scale>>, time: Res<Time>) {
    //(With<MainMenu>, With<Scale>)>) {
    for mut transform in &mut items {
        if transform.scale.x >= 1.1 {
            // do nothing
        } else {
            transform.scale += 1.5 * time.delta_seconds();
        }
        //transform.scale = Vec3::new(1.0, 1.0, 1.0);
    }
}

pub fn remove_hover(mut items: Query<&mut Transform, With<RevertScale>>, time: Res<Time>) {
    for mut transform in &mut items {
        if transform.scale.x <= 1.0 {
            // do nothing
        } else {
            transform.scale -= 1.5 * time.delta_seconds();
        }
        //transform.scale = Vec3::new(1.0, 1.0, 1.0);
    }
}

pub fn handle_settings_interactions(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    query: Query<(&Interaction, &SettingsItem, Entity)>,
    focused: Query<(
        &Interaction,
        &SettingsItem,
        Entity,
        With<Scale>,
        Without<KeyFocus>,
    )>,
    mut commands: Commands,
) {
    query.for_each(|(interaction, item, entity)| match interaction {
        Interaction::Clicked => match item {
            SettingsItem::Resume => {
                app_state
                    .push(GameState::GameLoop)
                    .map_err(|err| error!("Failed to start game: {}", err))
                    .unwrap();
            }
            SettingsItem::MainMenu => {
                app_state
                    .push(GameState::MainMenu)
                    .map_err(|err| error!("Failed to open settings: {}", err))
                    .unwrap();
            }
            SettingsItem::Quit => app_exit_events.send(AppExit),
        },
        // Handle Hover and remove hover events
        Interaction::Hovered => {
            // do nothing
            match item {
                SettingsItem::Resume => {
                    // do nothing for now
                    commands.entity(entity).remove::<RevertScale>();
                    commands.entity(entity).remove::<KeyFocus>();
                    commands.entity(entity).insert(Scale);
                }
                SettingsItem::MainMenu => {
                    // do nothing for now
                    commands.entity(entity).remove::<RevertScale>();
                    commands.entity(entity).remove::<KeyFocus>();
                    commands.entity(entity).insert(Scale);
                }
                SettingsItem::Quit => {
                    // do nothing for now
                    commands.entity(entity).remove::<RevertScale>();
                    commands.entity(entity).remove::<KeyFocus>();
                    commands.entity(entity).insert(Scale);
                }
            }
        }
        _default => {
            // do nothing
        }
    });
    focused.for_each(
        |(interaction, item, entity, _focus, _key)| match interaction {
            Interaction::Clicked => {
                // do nothing
            }
            Interaction::Hovered => {
                // do nothing
            }
            _default => {
                match item {
                    SettingsItem::Resume => {
                        // do nothing for now
                        commands.entity(entity).remove::<Scale>();
                        commands.entity(entity).insert(RevertScale);
                    }
                    SettingsItem::MainMenu => {
                        // do nothing for now
                        commands.entity(entity).remove::<Scale>();
                        commands.entity(entity).insert(RevertScale);
                    }
                    SettingsItem::Quit => {
                        // do nothing for now
                        commands.entity(entity).remove::<Scale>();
                        commands.entity(entity).insert(RevertScale);
                    }
                }
            }
        },
    )
}

pub fn navigate_settings_with_keys(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    mut key_event: EventReader<KeyboardInput>,
    buttons: Query<(&SettingsItem, Entity, &Grid)>,
    focused: Query<(&SettingsItem, &Grid, With<KeyFocus>)>,
    mut commands: Commands,
) {
    for key in key_event.iter() {
        let mut focused_button_pos: Vec<i32> = vec![0, -1];
        focused.for_each(|(_item, grid_pos, _key)| {
            focused_button_pos = vec![grid_pos.x, grid_pos.y];
        });
        match key.state {
            ButtonState::Pressed => {
                let code = key.key_code.unwrap_or(KeyCode::Backslash);
                match code {
                    KeyCode::Tab => {
                        buttons.iter().for_each(|(_item, entity, grid_pos)| {
                            // do nothing
                            if focused_button_pos[1] == 2 {
                                if grid_pos.y == 0 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            } else {
                                if grid_pos.y == focused_button_pos[1] + 1 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            }
                            if grid_pos.y == focused_button_pos[1] {
                                commands.entity(entity).remove::<KeyFocus>();
                            }
                        })
                    }
                    KeyCode::Up => {
                        buttons.iter().for_each(|(_item, entity, grid_pos)| {
                            // do nothing
                            if focused_button_pos[1] == 0 {
                                if grid_pos.y == 2 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            } else {
                                if grid_pos.y == focused_button_pos[1] - 1 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            }
                            if grid_pos.y == focused_button_pos[1] {
                                commands.entity(entity).remove::<KeyFocus>();
                            }
                        })
                    }
                    KeyCode::Down => {
                        buttons.iter().for_each(|(_item, entity, grid_pos)| {
                            // do nothing
                            if focused_button_pos[1] == 2 {
                                if grid_pos.y == 0 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            } else {
                                if grid_pos.y == focused_button_pos[1] + 1 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            }
                            if grid_pos.y == focused_button_pos[1] {
                                commands.entity(entity).remove::<KeyFocus>();
                            }
                        })
                    }
                    KeyCode::Return | KeyCode::Space => {
                        focused.for_each(|(item, _grid, _key)| match item {
                            SettingsItem::Resume => {
                                app_state
                                    .push(GameState::GameLoop)
                                    .map_err(|err| error!("Failed to start game: {}", err))
                                    .unwrap();
                            }
                            SettingsItem::MainMenu => {
                                app_state
                                    .push(GameState::MainMenu)
                                    .map_err(|err| error!("Failed to open settings: {}", err))
                                    .unwrap();
                            }
                            SettingsItem::Quit => app_exit_events.send(AppExit),
                        })
                    }
                    _default => {
                        // do nothing
                    }
                }
            }
            _default => {
                // do nothing
            }
        }
    }
}

pub fn spawn_setting_button(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    setting_item: SettingsItem,
    y_val: i32,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(15.0),
                    height: Val::Px(30.0),
                },

                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(setting_item)
        .insert(Grid { x: 0, y: y_val })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style::default(),
                text: Text::from_section(
                    match setting_item {
                        SettingsItem::Resume => "Resume",
                        SettingsItem::MainMenu => "Main Menu",
                        SettingsItem::Quit => "Quit",
                    },
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::DARK_GRAY,
                    },
                ),
                ..default()
            });
        });
}

pub fn init_settings(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    let font: Handle<Font> = asset_server.load("fonts/MajorMonoDisplay-Regular.ttf");

    clear_color.0 = Color::MIDNIGHT_BLUE;

    //let variants = mem::variant_count::<SettingsItem>();

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },

                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            color: UiColor(Color::OLIVE),
            ..NodeBundle::default()
        })
        .insert(SettingsMenu)
        .with_children(|mut parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Settings (More Coming Soon..)",
                    TextStyle {
                        font: font.clone(),
                        font_size: 35.0,
                        color: Color::WHITE,
                    },
                ),
                ..TextBundle::default()
            });

            spawn_setting_button(&mut parent, font.clone(), SettingsItem::Resume, 0);
            spawn_setting_button(&mut parent, font.clone(), SettingsItem::MainMenu, 1);
            spawn_setting_button(&mut parent, font.clone(), SettingsItem::Quit, 2);
        });
}
