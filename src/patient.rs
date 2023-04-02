use crate::loading::TextureAssets;
use crate::GameState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy_rapier2d::prelude::*;

pub struct PatientPlugin;

#[derive(Component)]
pub struct Patient;

/// This plugin handles patient related stuff like movement
/// Patient logic is only active during the State `GameState::Playing`
impl Plugin for PatientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_patient.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_patient.in_set(OnUpdate(GameState::Playing)))
            .add_system(handle_mouse.in_set(OnUpdate(GameState::Playing)));
    }
}

fn handle_mouse(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = primary_window.single();

    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                println!("Mouse button release: {:?}", ev.button);
                if let Some(position) = window.cursor_position() {
                    println!("Mouse released @ {:?}", position);
                }
            }
        }
    }
}

fn spawn_patient(mut commands: Commands, textures: Res<TextureAssets>) {
    for key in textures.folder.keys() {
        println!("found text: {}", key);
    }

    commands
        .spawn(SpriteBundle {
            texture: textures
                .folder
                .get("textures/patient_0.png")
                .unwrap()
                .clone(),
            transform: Transform::from_translation(Vec3::new(1., 400., 1.)),
            ..Default::default()
        })
        .insert(Patient)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(75.0))
        .insert(ColliderMassProperties::Density(2.0))
        .insert(Restitution::coefficient(0.7))
        .insert(ExternalForce {
            force: Vec2::new(100.0, 200.0),
            torque: 140.0,
        });
}

fn move_patient(
    time: Res<Time>,
    mut patient_query: Query<&mut Transform, With<Patient>>,
    mut ext_forces: Query<&mut ExternalForce>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
) {
}
