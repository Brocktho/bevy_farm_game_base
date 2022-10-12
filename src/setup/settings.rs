use crate::controls::menu::{handle_settings_interactions, navigate_settings_with_keys};
use crate::globals::ui_modifiers::{RevertScale, Scale, UiGrid};
use crate::states::GameState;
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
        .insert(UiGrid { x: 0, y: y_val })
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
    let background: Handle<Image> = asset_server.load("images/FarmHome.png");

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
            image: UiImage(background),
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
