use crate::controls::menu::{handle_main_menu_interactions, navigate_main_menu_with_keys};
use crate::globals::ui_modifiers::{RevertScale, Scale, UiGrid};
use crate::states::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Clone, Copy, Component)]
pub enum MenuItem {
    Play,
    Controls,
    Exit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(main_menu))
            .add_system_set(SystemSet::on_resume(GameState::MainMenu).with_system(main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(handle_main_menu_interactions),
            )
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(hover_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(remove_hover))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(navigate_main_menu_with_keys),
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
        .insert(UiGrid { x: 0, y: y_val })
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
    let background: Handle<Image> = asset_server.load("images/FarmHome.png");
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
            image: UiImage(background),
            ..NodeBundle::default()
        })
        .insert(MainMenu)
        .with_children(|mut parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Fast Paced Farming Game",
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
