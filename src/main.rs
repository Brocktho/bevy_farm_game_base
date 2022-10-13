use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
mod controls;
mod gameplay_systems;
mod globals;
mod setup;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(setup::menu::MainMenuPlugin)
        .add_plugin(setup::scene::GameScenePlugin)
        .add_plugin(setup::settings::SettingsPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<globals::character_modifiers::PlayerData>::new())
        .add_system(controls::controller::gamepad_connections)
        .add_plugin(HanabiPlugin)
        .add_startup_system(setup::camera::ui_camera)
        .add_state(states::GameState::MainMenu)
        .run();
}
