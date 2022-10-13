use bevy::prelude::*;

#[derive(Component)]
pub struct UiGrid {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Scale;

#[derive(Component)]
pub struct RevertScale;

#[derive(Component)]
pub struct KeyFocus;

#[derive(Component)]
pub struct Inventory;
