use std::time::Duration;

use bevy::prelude::*;
use rand::{distributions::Uniform, Rng};

use crate::WINDOW_SIZE;

const PLAYER_SIZE: f32 = WINDOW_SIZE.0 / 3.;
const PLAYER_SPEED: f32 = 1500.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    MainMenu,
    Playing,
    Paused,
}

pub(crate) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Playing);
        // .add_startup_system(startup);

        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_game));

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(spawn_obstacles)
                .with_system(move_obstacles)
                .with_system(handle_input)
                .with_system(move_player),
        );
    }
}

// fn startup(mut state: ResMut<State<GameState>>) {
//     // if let Err(e) = state.set(GameState::Playing) {
//     //     warn!("Unable to set game state at start");
//     //     warn!("{}", e);
//     // }
// }

//--Components--//
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Obstacle;
#[derive(Component, Copy, Clone)]
enum Lane {
    Left = -1,
    Middle = 0,
    Right = 1,
}

impl From<i8> for Lane {
    fn from(o: i8) -> Self {
        match o {
            x if x < 0 => Self::Left,
            0 => Self::Middle,
            x if x > 0 => Self::Right,
            _ => panic!("Uhhh this should be unreachable"),
        }
    }
}

//--RESOURCES--//
struct ObsTimer(Timer);

fn setup_game(mut commands: Commands) {
    commands.insert_resource(ObsTimer(Timer::new(Duration::from_secs(1), true)));
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn()
        .insert(Player)
        .insert(Lane::Middle)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::CYAN,
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -300. + PLAYER_SIZE, 0.)),
            ..default()
        });
}

fn cycle_lanes(lane: &mut Lane, target: Lane) {
    let val = (*lane as i8) + (target as i8);
    *lane = val.into();
}

//--SYSTEMS--//

fn handle_input(keys: Res<Input<KeyCode>>, mut q: Query<&mut Lane, With<Player>>) {
    if let Ok(mut lane) = q.get_single_mut() {
        if keys.any_just_pressed([KeyCode::D, KeyCode::Right]) {
            cycle_lanes(&mut lane, Lane::Right);
        } else if keys.any_just_pressed([KeyCode::A, KeyCode::Left]) {
            cycle_lanes(&mut lane, Lane::Left);
        }
    }
}

fn move_player(
    mut tween: Local<f32>,
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Lane), With<Player>>,
) {
    if let Ok((mut tr, lane)) = q.get_single_mut() {
        let target = (PLAYER_SIZE) * (*lane as i8 as f32);
        if *tween <= target + 10. && *tween >= target - 10. {
            *tween = target
        } else {
            *tween -= ((*tween - target).signum() * PLAYER_SPEED) * (time.delta_seconds());
            info!("tween is: {}", *tween);
        }
        tr.translation.x = *tween;
    }
}

fn spawn_obstacles(mut timer: ResMut<ObsTimer>, time: Res<Time>, mut commands: Commands) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut rng = rand::thread_rng();
        let lanes = Uniform::new_inclusive(-1i8, 1i8);
        let lane: Lane = rng.sample(lanes).into();
        commands
            .spawn()
            .insert(Obstacle)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(PLAYER_SIZE * lane as i8 as f32, 700., 0.),
                ..default()
            });
    }
}

fn move_obstacles(mut q: Query<&mut Transform, With<Obstacle>>, time: Res<Time>) {
    for mut tr in q.iter_mut() {
        tr.translation.y -= 250. * time.delta_seconds();
    }
}
