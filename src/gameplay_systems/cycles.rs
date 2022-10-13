use bevy::prelude::*;

pub struct CyclePlugin;

impl Plugin for CyclePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct EnemyTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct CycleTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct DayTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct NightTimer {
    pub timer: Timer,
}
