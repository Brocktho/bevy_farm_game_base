use bevy::prelude::*;

#[derive(Component)]
pub struct GameGrid {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Focus;

#[derive(Component)]
pub struct FarmLand;
