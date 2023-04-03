use crate::loading::TextureAssets;
use crate::GameState;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::cubic_splines::Point;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy_rapier2d::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::RngCore;
use rand::{rngs::StdRng, SeedableRng};

use crate::pill::SpawnPillEvent;

#[derive(Component)]
pub struct Beaker;

pub struct BeakerPlugin;

impl Plugin for BeakerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_beakers.in_schedule(OnEnter(GameState::Playing)))
            .add_system(handle_mouse.in_set(OnUpdate(GameState::Playing)));
    }
}

const beaker_scale: f32 = 0.4;

fn spawn_beakers(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    assets: Res<Assets<Image>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let text = textures.folder.get("textures/beaker.png").unwrap();
    let window = window_q.single();

    let margin: f32 = 30.;
    let x: f32 = window.resolution.width() / 2. - margin;
    let y: f32 = window.resolution.height() / 2. - margin;
    let z: f32 = 1.;
    let r: f32 = 30.;

    let pos = vec![
        // Left side, bottom-to-top
        (-x, -y, 0., -r / 2.),
        (-x, 0., 0., -r),
        (-x, y, 0., -r * 2.),
        // Right side, bottom-to-top
        (x, -y, 0., r / 2.),
        (x, 0., 0., r),
        (x, y, 0., r * 2.),
    ];

    for pos in pos {
        commands
            .spawn(SpriteBundle {
                texture: text.clone(),
                transform: Transform::from_xyz(pos.0, pos.1, pos.2)
                    .with_scale(Vec3::new(beaker_scale, beaker_scale, 1.))
                    .with_rotation(Quat::from_rotation_z((pos.3).to_radians())),
                ..Default::default()
            })
            .insert(Beaker)
            .insert(RigidBody::Fixed)
            .insert(Collider::ball(100.0));
    }
}

fn handle_mouse(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut ev_spawn_pill: EventWriter<SpawnPillEvent>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let window = primary_window.single();
    let (camera, camera_transform) = camera_q.single();

    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                println!("Mouse button release: {:?}", ev.button);
                if let Some(pos) = window.cursor_position() {
                    println!("Mouse released @ {:?}", pos);
                }
            }
        }
    }
}
