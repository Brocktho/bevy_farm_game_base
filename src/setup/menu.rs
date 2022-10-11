use crate::states::base::*;
use bevy::app::AppExit;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::{Camera2dBundle, Commands, *};
use bevy::ui::UiColor;

pub fn ui_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle { ..default() });
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Clone, Copy, Component)]
pub enum MenuItem {
    Play,
    Controls,
    Exit,
}

#[derive(Component)]
pub struct Grid {
    x: i32,
    y: i32,
}
#[derive(Component)]
pub struct Scale;

#[derive(Component)]
pub struct RevertScale;

#[derive(Component)]
pub struct KeyFocus;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(main_menu))
            .add_system_set(SystemSet::on_resume(GameState::MainMenu).with_system(main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(handle_menu_interactions),
            )
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(hover_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(remove_hover))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(navigate_with_key),
            )
            .add_system_set(SystemSet::on_pause(GameState::MainMenu).with_system(remove_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(remove_menu));
    }
}

pub fn remove_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
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

pub fn handle_menu_interactions(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    query: Query<(&Interaction, &MenuItem, Entity)>,
    focused: Query<(
        &Interaction,
        &MenuItem,
        Entity,
        With<Scale>,
        Without<KeyFocus>,
    )>,
    mut commands: Commands,
) {
    query.for_each(|(interaction, item, entity)| match interaction {
        Interaction::Clicked => match item {
            MenuItem::Play => {
                app_state
                    .push(GameState::GameLoop)
                    .map_err(|err| error!("Failed to start game: {}", err))
                    .unwrap();
            }
            MenuItem::Controls => {
                app_state
                    .push(GameState::Settings)
                    .map_err(|err| error!("Failed to open settings: {}", err))
                    .unwrap();
            }
            MenuItem::Exit => app_exit_events.send(AppExit),
        },
        // Handle Hover and remove hover events
        Interaction::Hovered => {
            // do nothing
            match item {
                MenuItem::Play => {
                    // do nothing for now
                    commands.entity(entity).remove::<RevertScale>();
                    commands.entity(entity).remove::<KeyFocus>();
                    commands.entity(entity).insert(Scale);
                }
                MenuItem::Controls => {
                    // do nothing for now
                    commands.entity(entity).remove::<RevertScale>();
                    commands.entity(entity).remove::<KeyFocus>();
                    commands.entity(entity).insert(Scale);
                }
                MenuItem::Exit => {
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
                    MenuItem::Play => {
                        // do nothing for now
                        commands.entity(entity).remove::<Scale>();
                        commands.entity(entity).insert(RevertScale);
                    }
                    MenuItem::Controls => {
                        // do nothing for now
                        commands.entity(entity).remove::<Scale>();
                        commands.entity(entity).insert(RevertScale);
                    }
                    MenuItem::Exit => {
                        // do nothing for now
                        commands.entity(entity).remove::<Scale>();
                        commands.entity(entity).insert(RevertScale);
                    }
                }
            }
        },
    )
}

pub fn navigate_with_key(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    mut key_event: EventReader<KeyboardInput>,
    buttons: Query<(&MenuItem, Entity, &Grid)>,
    focused: Query<(&MenuItem, &Grid, With<KeyFocus>)>,
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
                            MenuItem::Play => {
                                app_state
                                    .push(GameState::GameLoop)
                                    .map_err(|err| error!("Failed to start game: {}", err))
                                    .unwrap();
                            }
                            MenuItem::Controls => {
                                app_state
                                    .push(GameState::Settings)
                                    .map_err(|err| error!("Failed to open settings: {}", err))
                                    .unwrap();
                            }
                            MenuItem::Exit => app_exit_events.send(AppExit),
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

pub fn spawn_main_button(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    menu_item: MenuItem,
    y_val: i32,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(10.0),
                    height: Val::Px(30.0),
                },

                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(menu_item)
        .insert(Grid { x: 0, y: y_val })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style::default(),
                text: Text::from_section(
                    match menu_item {
                        MenuItem::Play => "Play",
                        MenuItem::Controls => "Controls",
                        MenuItem::Exit => "Exit",
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

pub fn main_menu(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    let font: Handle<Font> = asset_server.load("fonts/MajorMonoDisplay-Regular.ttf");

    clear_color.0 = Color::MIDNIGHT_BLUE;

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
            color: UiColor(Color::MIDNIGHT_BLUE),
            ..NodeBundle::default()
        })
        .insert(MainMenu)
        .with_children(|mut parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Who Cares",
                    TextStyle {
                        font: font.clone(),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                ),
                ..TextBundle::default()
            });

            spawn_main_button(&mut parent, font.clone(), MenuItem::Play, 0);
            spawn_main_button(&mut parent, font.clone(), MenuItem::Controls, 1);
            spawn_main_button(&mut parent, font.clone(), MenuItem::Exit, 2);
        });
}
