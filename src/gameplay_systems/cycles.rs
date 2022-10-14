use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

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

#[derive(Inspectable)]
pub struct TimersData {
    #[inspectable(min = 1.0, max = 15.0)]
    pub enemy_times: f32,
    #[inspectable(min = 20.0, max = 100.0)]
    pub night_spawn_percent: f32,
    #[inspectable(min = 90.0, max = 400.0)]
    pub cycle_times: f32,
}

impl Default for TimersData {
    fn default() -> Self {
        TimersData {
            enemy_times: 3.0,
            night_spawn_percent: 80.0,
            cycle_times: 120.0,
        }
    }
}

//pub fn update_timers_gui()
