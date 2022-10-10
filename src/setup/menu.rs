use crate::states::base::GameState;
use bevy::app::AppExit;
use bevy::prelude::{Camera2dBundle, Commands, *};

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

pub fn spawn_button(parent: &mut ChildBuilder, font: Handle<Font>, menu_item: MenuItem) {
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

            spawn_button(&mut parent, font.clone(), MenuItem::Play);
            spawn_button(&mut parent, font.clone(), MenuItem::Controls);
            spawn_button(&mut parent, font.clone(), MenuItem::Exit);
        });
}
