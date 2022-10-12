use std::time::Duration;

use super::menu::MyCamera;
use crate::states::base::GameState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_hanabi::*;
use rand::Rng;
pub struct GameScenePlugin;

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Player {
    //health: i32,
    max_speed: f32,
    acceleration: f32,
    current_speed: Vec3,
    location: Vec3,
}

#[derive(Component)]
pub struct EnemyTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Enemy {
    max_speed: f32,
    acceleration: f32,
    current_speed: Vec3,
    x_direction: bool, // true was last traveling in positive x direction
    y_direction: bool, // true was last traveling in positive y direction
}

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameLoop).with_system(initialize_game))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_character))
            .add_system_set(
                SystemSet::on_update(GameState::GameLoop).with_system(handle_menu_particles),
            )
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_camera))
            .add_system_set(SystemSet::on_update(GameState::GameLoop).with_system(move_enemies))
            .add_system_set(
                SystemSet::on_update(GameState::GameLoop)
                    .after(initialize_game)
                    .with_system(spawn_enemies),
            );
    }
}

pub fn move_enemies(
    players: Query<&Player>,
    mut enemies: Query<(&mut Transform, &mut Enemy, With<Enemy>)>,
    time: Res<Time>,
) {
    if !players.is_empty() {
        let player = players.iter().next().unwrap();
        let target = player.location;
        enemies
            .iter_mut()
            .for_each(|(mut transform, mut stats, _search)| {
                let dx = target.x - transform.translation.x;
                let dy = target.y - transform.translation.y;
                if dx > 0.0 {
                    if !stats.x_direction {
                        stats.current_speed.x = 0.0;
                    }
                    if stats.current_speed.x < stats.max_speed {
                        stats.current_speed.x += stats.acceleration * time.delta_seconds();
                    }
                    transform.translation.x += stats.current_speed.x * time.delta_seconds();
                    stats.x_direction = true;
                } else if dx < 0.0 {
                    if stats.x_direction {
                        stats.current_speed.x = 0.0;
                    }
                    if stats.current_speed.x < stats.max_speed {
                        stats.current_speed.x += stats.acceleration * time.delta_seconds();
                    }
                    transform.translation.x -= stats.current_speed.x * time.delta_seconds();
                    stats.x_direction = false;
                }

                if dy > 0.0 {
                    if !stats.y_direction {
                        stats.current_speed.y = 0.0;
                    }
                    if stats.current_speed.y < stats.max_speed {
                        stats.current_speed.y += stats.acceleration * time.delta_seconds();
                    }
                    transform.translation.y += stats.current_speed.y * time.delta_seconds();
                    stats.y_direction = true;
                } else if dy < 0.0 {
                    if stats.y_direction {
                        stats.current_speed.y = 0.0;
                    }
                    if stats.current_speed.y < stats.max_speed {
                        stats.current_speed.y += stats.acceleration * time.delta_seconds();
                    }
                    transform.translation.y -= stats.current_speed.y * time.delta_seconds();
                    stats.y_direction = false;
                }
            });
    }
}

pub fn initialize_game(
    mut commands: Commands,
    server: Res<AssetServer>,
    players: Query<Entity, With<Player>>,
) {
    let handle: Handle<Image> = server.load("images/Bunny.png");
    if players.is_empty() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: handle.clone(),
                transform: Transform {
                    translation: Vec3::splat(0.0),
                    scale: Vec3::new(0.25, 0.25, 0.25),
                    //translation:
                    ..default()
                },
                ..default()
            })
            .insert(Player {
                //health: 3,
                max_speed: 50.0,
                acceleration: 75.0,
                current_speed: Vec3::splat(0.0),
                location: Vec3::splat(0.0),
            });
        commands.insert_resource(UiImage(handle));
        commands.insert_resource(EnemyTimer {
            timer: Timer::new(Duration::from_secs(3), true),
        });
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    server: Res<AssetServer>,
    enemies: Query<Entity, With<Enemy>>,
    player_query: Query<&Player>,
    mut enemy_timer: ResMut<EnemyTimer>,
    time: Res<Time>,
) {
    const DISTANCE: f32 = 50.0;
    if !player_query.is_empty() {
        if enemies.iter().len() < 256 {
            enemy_timer.timer.tick(time.delta());
            if enemy_timer.timer.finished() {
                let player = player_query.iter().next().unwrap();
                let enemy_sprite = server.load("images/Zombie.png");
                let mut rng = rand::thread_rng();
                let x_off: f32 = (rng.gen::<f32>() * DISTANCE) + DISTANCE;
                let y_off: f32 = (rng.gen::<f32>() * DISTANCE) + DISTANCE;
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: enemy_sprite.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                player.location.x + x_off,
                                player.location.y + y_off,
                                player.location.z,
                            ),
                            scale: Vec3::new(0.15, 0.15, 0.15),
                            rotation: Quat::from_rotation_z(150.0),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Enemy {
                        max_speed: 20.0,
                        acceleration: 40.0,
                        current_speed: Vec3::splat(0.0),
                        x_direction: false,
                        y_direction: false,
                    });
            }
        }
    }
}

pub fn handle_menu_particles(
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>, //asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    particle_sources: Query<(Entity, With<Particle>)>,
    window: Res<Windows>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = window.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::splat(0.2),
                            ..Default::default()
                        }))
                        .into(),
                    material: materials.add(ColorMaterial {
                        color: Color::WHITE,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .insert(Name::new("square"))
                .insert(Particle);

            // Create a color gradient for the particles
            let mut gradient = Gradient::new();
            gradient.add_key(0.0, Vec4::new(0.9, 0.3, 0.2, 1.0));
            gradient.add_key(0.2, Vec4::new(0.2, 0.7, 0.2, 0.0));
            gradient.add_key(0.4, Vec4::new(0.4, 0.4, 0.2, 0.0));
            gradient.add_key(0.6, Vec4::new(0.1, 0.1, 0.6, 0.0));
            gradient.add_key(0.8, Vec4::new(0.7, 0.2, 1.0, 0.0));
            // Create a new effect asset spawning 30 particles per second from a circle
            // and slowly fading from blue-ish to transparent over their lifetime.
            // By default the asset spawns the particles at Z=0.
            let spawner = Spawner::rate(30.0.into());
            let effect = effects.add(
                EffectAsset {
                    name: "Effect".into(),
                    capacity: 4096,
                    spawner,
                    ..Default::default()
                }
                .init(PositionCircleModifier {
                    //center: Vec3::new(pos.x, pos.y, default()),
                    //axis: Vec3::Z,
                    radius: 0.2,
                    speed: 0.1.into(),
                    dimension: ShapeDimension::Surface,
                    ..Default::default()
                })
                .render(SizeOverLifetimeModifier {
                    gradient: Gradient::constant(Vec2::splat(0.2)),
                })
                .render(ColorOverLifetimeModifier { gradient }),
            );

            // Spawn an instance of the particle effect, and override its Z layer to
            // be above the reference white square previously spawned.
            commands
                .spawn_bundle(ParticleEffectBundle {
                    // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
                    effect: ParticleEffect::new(effect).with_z_layer_2d(Some(0.1)),
                    ..default()
                })
                .insert(Name::new("effect:2d"))
                .insert(Particle);
        }
    }
    if buttons.just_released(MouseButton::Left) {
        particle_sources.iter().for_each(|(entity, _key)| {
            println!("Should despawn: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        })
    }
}

pub fn move_camera(
    mut camera: Query<(&mut Transform, &mut MyCamera, With<MyCamera>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if keys.pressed(KeyCode::Minus) {
        for (mut transform, _cam, _search) in &mut camera {
            transform.scale += 5.0 * time.delta_seconds();
            println!("Camera scale: {:?}", transform.scale);
        }
    }
    if keys.pressed(KeyCode::Equals) {
        for (mut transform, _cam, _search) in &mut camera {
            transform.scale -= 5.0 * time.delta_seconds();
            println!("Camera scale: {:?}", transform.scale);
        }
    }
}

pub fn move_character(
    mut _commands: Commands,
    mut character: Query<(&mut Transform, &mut Player, With<Player>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keys.pressed(KeyCode::Z) {
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.z {
                player.current_speed.z += player.acceleration * time.delta_seconds();
            }
            transform.translation.z += player.current_speed.z * time.delta_seconds();
            player.location = transform.translation;
            println!("Z index: {:?}", transform);
        }
    }
    if keys.pressed(KeyCode::X) {
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.z {
                player.current_speed.z += player.acceleration * time.delta_seconds();
            }
            transform.translation.z -= player.current_speed.z * time.delta_seconds();
            player.location = transform.translation;
            println!("Z index: {:?}", transform);
        }
    }
    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
        // up
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.y {
                player.current_speed.y += player.acceleration * time.delta_seconds();
            }
            transform.translation.y += player.current_speed.y * time.delta_seconds();
            player.location = transform.translation;
        }
    }
    if keys.just_released(KeyCode::W) || keys.just_released(KeyCode::Up) {
        for (mut _transform, mut player, _with) in &mut character {
            player.current_speed.y = 0.0;
        }
    }
    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
        // left
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.x {
                player.current_speed.x += player.acceleration * time.delta_seconds();
            }
            transform.translation.x -= player.current_speed.x * time.delta_seconds();
            player.location = transform.translation;
        }
    }
    if keys.just_released(KeyCode::A) || keys.just_released(KeyCode::Left) {
        for (mut _transform, mut player, _with) in &mut character {
            player.current_speed.x = 0.0;
        }
    }
    if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
        // down
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.y {
                player.current_speed.y += player.acceleration * time.delta_seconds();
            }
            transform.translation.y -= player.current_speed.y * time.delta_seconds();
            player.location = transform.translation;
        }
    }
    if keys.just_released(KeyCode::S) || keys.just_released(KeyCode::Down) {
        for (mut _transform, mut player, _with) in &mut character {
            player.current_speed.y = 0.0;
        }
    }
    if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
        // right
        for (mut transform, mut player, _with) in &mut character {
            if player.max_speed > player.current_speed.x {
                player.current_speed.x += player.acceleration * time.delta_seconds();
            }
            transform.translation.x += player.current_speed.x * time.delta_seconds();
            player.location = transform.translation;
        }
    }
    if keys.just_released(KeyCode::D) || keys.just_released(KeyCode::Right) {
        for (mut _transform, mut player, _with) in &mut character {
            player.current_speed.x = 0.0;
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        game_state
            .push(GameState::Settings)
            .map_err(|err| error!("Unable to open Settings: {}", err))
            .unwrap();
    }
}
