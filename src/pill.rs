use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::cubic_splines::Point;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy_rapier2d::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::{rngs::StdRng, SeedableRng};

pub struct PillPlugin;

#[derive(Component)]
pub struct Pill;

struct SpawnPillEvent(Vec2);

impl Plugin for PillPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_pill.in_set(OnUpdate(GameState::Playing)))
            .add_system(handle_mouse.in_set(OnUpdate(GameState::Playing)))
            .add_system(spawn_pills.in_set(OnUpdate(GameState::Playing)))
            .add_event::<SpawnPillEvent>();
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

                    if let Some(world_position) = window
                        .cursor_position()
                        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                        .map(|ray| ray.origin.truncate())
                    {
                        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
                        ev_spawn_pill.send(SpawnPillEvent(world_position));
                    }
                }
            }
        }
    }
}

fn spawn_pills(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut ev_spawn_pill: EventReader<SpawnPillEvent>,
) {
    let mut rng = StdRng::from_entropy();
    for ev in ev_spawn_pill.iter() {
        let mut points = Vec::new();

        let (x_scale, y_scale) = (150., 300.);
        for _ in 0..10 {
            let x: f32 = Standard.sample(&mut rng);
            let y: f32 = Standard.sample(&mut rng);
            points.push(Vect::new(
                x * x_scale - x_scale / 2.,
                y * y_scale - y_scale / 2.,
            ));
        }

        let force_scale = 3000.;
        let torque_scale = 20.;

        let x_force_sample: f32 = Standard.sample(&mut rng);
        let y_force_sample: f32 = Standard.sample(&mut rng);
        let torque_sample: f32 = Standard.sample(&mut rng);

        let x_force = x_force_sample * force_scale - force_scale / 2.;
        let y_force = y_force_sample * force_scale - force_scale / 2.;
        let torque_force = torque_sample * torque_scale - torque_scale / 2.;

        commands
            .spawn(SpriteBundle {
                texture: textures.folder.get("textures/pill_0.png").unwrap().clone(),
                transform: Transform::from_translation(Vec3::new(ev.0.x, ev.0.y, 1.))
                    .with_scale(Vec3::new(0.5, 0.5, 1.)),
                ..Default::default()
            })
            .insert(Pill)
            .insert(RigidBody::Dynamic)
            // .insert(Collider::ball(60.0))
            .insert(Collider::convex_hull(&points).unwrap())
            .insert(ColliderMassProperties::Density(1.))
            .insert(Restitution::coefficient(0.9))
            .insert(ExternalForce {
                force: Vec2::new(x_force, y_force),
                torque: torque_force,
            });
    }
}

fn move_pill(
    time: Res<Time>,
    actions: Res<Actions>,
    mut pill_query: Query<&mut Transform, With<Pill>>,
    mut ext_forces: Query<&mut ExternalForce>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
) {
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = Vec2::new(0., 0.0);
        ext_force.torque = 0.0;
    }
}
