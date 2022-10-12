use crate::globals::ui_modifiers::{KeyFocus, RevertScale, Scale, UiGrid};
use crate::setup::menu::MenuItem;
use crate::setup::settings::SettingsItem;
use crate::states::GameState;
use bevy::app::AppExit;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

// Main Menu Navigation
pub fn handle_main_menu_interactions(
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

pub fn navigate_main_menu_with_keys(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    mut key_event: EventReader<KeyboardInput>,
    buttons: Query<(&MenuItem, Entity, &UiGrid)>,
    focused: Query<(&MenuItem, &UiGrid, With<KeyFocus>)>,
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

// Settings Navigation ( Almost the same as Main Menu Navigation, but query for SettingsItems instead ) Will need a lot of changes in the future.
pub fn navigate_settings_with_keys(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    mut key_event: EventReader<KeyboardInput>,
    buttons: Query<(&SettingsItem, Entity, &UiGrid)>,
    focused: Query<(&SettingsItem, &UiGrid, With<KeyFocus>)>,
    mut commands: Commands,
) {
    for key in key_event.iter() {
        let mut focused_button_pos: Vec<i32> = vec![0, -1];
        focused.for_each(|(_item, ui_grid_pos, _key)| {
            focused_button_pos = vec![ui_grid_pos.x, ui_grid_pos.y];
        });
        match key.state {
            ButtonState::Pressed => {
                let code = key.key_code.unwrap_or(KeyCode::Backslash);
                match code {
                    KeyCode::Tab => {
                        buttons.iter().for_each(|(_item, entity, ui_grid_pos)| {
                            // do nothing
                            if focused_button_pos[1] == 2 {
                                if ui_grid_pos.y == 0 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            } else {
                                if ui_grid_pos.y == focused_button_pos[1] + 1 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            }
                            if ui_grid_pos.y == focused_button_pos[1] {
                                commands.entity(entity).remove::<KeyFocus>();
                            }
                        })
                    }
                    KeyCode::Up => {
                        buttons.iter().for_each(|(_item, entity, ui_grid_pos)| {
                            // do nothing
                            if focused_button_pos[1] == 0 {
                                if ui_grid_pos.y == 2 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            } else {
                                if ui_grid_pos.y == focused_button_pos[1] - 1 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            }
                            if ui_grid_pos.y == focused_button_pos[1] {
                                commands.entity(entity).remove::<KeyFocus>();
                            }
                        })
                    }
                    KeyCode::Down => {
                        buttons.iter().for_each(|(_item, entity, ui_grid_pos)| {
                            // do nothing
                            if focused_button_pos[1] == 2 {
                                if ui_grid_pos.y == 0 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            } else {
                                if ui_grid_pos.y == focused_button_pos[1] + 1 {
                                    commands.entity(entity).insert(KeyFocus);
                                    commands.entity(entity).insert(Scale);
                                    commands.entity(entity).remove::<RevertScale>();
                                }
                            }
                            if ui_grid_pos.y == focused_button_pos[1] {
                                commands.entity(entity).remove::<KeyFocus>();
                            }
                        })
                    }
                    KeyCode::Return | KeyCode::Space => {
                        focused.for_each(|(item, _ui_grid, _key)| match item {
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
