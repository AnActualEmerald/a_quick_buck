use bevy::prelude::*;

mod game;

const WINDOW_SIZE: (f32, f32) = (400., 600.);

pub fn run() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "A Quick Buck".to_string(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(game::Game)
        .run();
}
