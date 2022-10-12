use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// Player Zone
#[derive(Component)]
pub struct Player {
    //health: i32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub current_speed: Vec3,
    pub location: Vec3,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            max_speed: 50.0,
            acceleration: 75.0,
            current_speed: Vec3::splat(0.0),
            location: Vec3::splat(0.0),
        }
    }
}

// Player Inspection Zone:
#[derive(Inspectable)]
pub struct PlayerData {
    #[inspectable(min = 10.0, max = 150.0)]
    pub max_speed: f32,
    #[inspectable(min = 10.0, max = 300.0)]
    pub acceleration: f32,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            max_speed: 50.0,
            acceleration: 75.0,
        }
    }
}

pub fn update_player_values(data: Res<PlayerData>, mut query: Query<&mut Player>) {
    if !data.is_changed() {
        return;
    }
    for mut player in query.iter_mut() {
        player.max_speed = data.max_speed;
        player.acceleration = data.acceleration;
    }
}
// Enemy Zone ( could use any other systems if wanted )
#[derive(Inspectable, Component)]
pub struct Enemy {
    pub max_speed: f32,
    pub acceleration: f32,
    pub current_speed: Vec3,
    pub x_direction: bool, // true was last traveling in positive x direction
    pub y_direction: bool, // true was last traveling in positive y direction
}

/* pub const ZOMBIE: Enemy = Enemy {
    max_speed: 13.0,
    acceleration: 10.0,
    current_speed: Vec3::splat(0.0),
    x_direction: false,
    y_direction: false,
};

pub const FOX: Enemy = Enemy {
    max_speed: 15.0,
    acceleration: 20.0,
    current_speed: Vec3::splat(0.0),
    x_direction: false,
    y_direction: false,
};

pub const WOLF: Enemy = Enemy {
    max_speed: 30.0,
    acceleration: 45.0,
    current_speed: Vec3::splat(0.0),
    x_direction: false,
    y_direction: false,
}; */
