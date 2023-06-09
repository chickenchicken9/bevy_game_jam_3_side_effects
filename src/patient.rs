use crate::loading::TextureAssets;
use crate::GameState;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::pill::Pill;
use bevy_rapier2d::geometry::ActiveEvents;
use bevy_rapier2d::pipeline::CollisionEvent;
use rand::distributions::{Distribution, Standard};
use rand::RngCore;
use rand::{rngs::StdRng, SeedableRng};

pub struct PatientPlugin;
pub struct PatientHealedEvent;

#[derive(Component)]
pub struct Patient;

/// This plugin handles patient related stuff like movement
/// Patient logic is only active during the State `GameState::Playing`
impl Plugin for PatientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_patient_spawning.in_schedule(OnEnter(GameState::Playing)))
            .add_system(spawn_patient.in_set(OnUpdate(GameState::Playing)))
            .add_system(handle_collisions.in_set(OnUpdate(GameState::Playing)))
            .add_system(move_patient.in_set(OnUpdate(GameState::Playing)))
            .add_event::<PatientHealedEvent>();
    }
}

#[derive(Resource)]
struct PatientSpawnConfig {
    /// How often to spawn a new patient? (repeating timer)
    timer: Timer,
}

/// Configure our patient spawning algorithm
fn setup_patient_spawning(mut commands: Commands) {
    commands.insert_resource(PatientSpawnConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}

const PATIENT_SCALE: f32 = 0.5;
fn spawn_patient(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    time: Res<Time>,
    mut config: ResMut<PatientSpawnConfig>,
    assets: Res<Assets<Image>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if !config.timer.finished() {
        return;
    }

    let window = primary_window.single();
    let mut rng = StdRng::from_entropy();
    let text_path = format!("textures/patient_{}.png", rng.next_u32() % 4);
    let text = textures.folder.get(&text_path).unwrap();
    let img = assets.get(text).unwrap();

    let mut points = Vec::new();

    for _ in 0..10 {
        let x: f32 = Standard.sample(&mut rng);
        let y: f32 = Standard.sample(&mut rng);
        points.push(Vect::new(
            x * img.size().x - img.size().x / 2.,
            y * img.size().y - img.size().y / 2.,
        ));
    }

    // from paint - not convex, can't use :/
    let _patient_points = vec![
        (67, 24),
        (104, 48),
        (93, 85),
        (119, 111),
        (131, 261),
        (107, 296),
        (84, 290),
        (77, 267),
        (69, 296),
        (32, 298),
        (34, 287),
        (54, 282),
        (54, 272),
        (26, 272),
        (23, 248),
        (43, 97),
        (70, 87),
        (44, 65),
        (38, 43),
        (52, 27),
    ];

    let force_scale = 300.;
    let torque_scale = 5.;

    let x_force_sample: f32 = Standard.sample(&mut rng);
    let y_force_sample: f32 = Standard.sample(&mut rng);
    let torque_sample: f32 = Standard.sample(&mut rng);

    let x_force = x_force_sample * force_scale - force_scale / 2.;
    let y_force = y_force_sample * force_scale - force_scale / 2.;
    let torque_force = torque_sample * torque_scale - torque_scale / 2.;

    let x_pos_sample: f32 = Standard.sample(&mut rng);
    let y_pos_sample: f32 = Standard.sample(&mut rng);

    commands
        .spawn(SpriteBundle {
            texture: text.clone(),
            transform: Transform::from_translation(Vec3::new(
                (x_pos_sample - 0.5) * window.width(),
                (y_pos_sample - 0.5) * window.height(),
                1.,
            ))
            .with_scale(Vec3::new(PATIENT_SCALE, PATIENT_SCALE, 1.)),
            ..Default::default()
        })
        .insert(Patient)
        .insert(RigidBody::Dynamic)
        .insert(Collider::convex_hull(&points).unwrap())
        .insert(ColliderMassProperties::Density(2.0))
        .insert(Restitution::coefficient(0.7))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ExternalImpulse {
            impulse: Vec2::new(x_force, y_force),
            torque_impulse: torque_force,
        });
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    patients: Query<&Patient>,
    pills: Query<&Pill>,
    mut commands: Commands,
    mut ev_heal_pt: EventWriter<PatientHealedEvent>,
) {
    for ev in collision_events.iter() {
        match ev {
            CollisionEvent::Started(e1, e2, _flags) => {
                if let Ok(_pill) = pills.get(*e1) {
                    if let Ok(_patient) = patients.get(*e2) {
                        commands.entity(*e1).despawn_recursive();
                        commands.entity(*e2).despawn_recursive();
                        ev_heal_pt.send(PatientHealedEvent);
                    }
                }

                if let Ok(_pill) = pills.get(*e2) {
                    if let Ok(_patient) = patients.get(*e1) {
                        commands.entity(*e1).despawn_recursive();
                        commands.entity(*e2).despawn_recursive();
                        ev_heal_pt.send(PatientHealedEvent);
                    }
                }
            }
            _ => {}
        }
    }
}

fn move_patient(
    _time: Res<Time>,
    _patient_query: Query<&mut Transform, With<Patient>>,
    _ext_forces: Query<&mut ExternalForce>,
    _ext_impulses: Query<&mut ExternalImpulse>,
) {
}
