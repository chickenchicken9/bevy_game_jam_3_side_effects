mod audio;
mod beaker;
mod loading;
mod menu;
mod patient;
mod pill;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::beaker::BeakerPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::patient::PatientPlugin;
use crate::pill::PillPlugin;
use crate::ui::UiPlugin;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy_rapier2d::prelude::*;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PatientPlugin)
            .add_plugin(PillPlugin)
            .add_plugin(BeakerPlugin)
            .add_plugin(UiPlugin)
            // .add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_startup_system(setup_physics);
        // .add_system(print_ball_altitude);

        #[cfg(debug_assertions)]
        {
            app
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(RapierDebugRenderPlugin::default()) // shows collision boxes
                // .add_plugin(LogDiagnosticsPlugin::default())
                ;
        }
    }
}

fn setup_physics(mut commands: Commands, window_q: Query<&Window, With<PrimaryWindow>>) {
    let window = window_q.single();
    // floor
    commands
        .spawn(Collider::cuboid(500.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            -(window.resolution.height() / 2.),
            0.0,
        )));

    // left wall
    commands
        .spawn(Collider::cuboid(10.0, 500.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            -(window.resolution.width() / 2.),
            0.,
            0.0,
        )));

    // right wall
    commands
        .spawn(Collider::cuboid(10.0, 500.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            window.resolution.width() / 2.,
            0.,
            0.0,
        )));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
