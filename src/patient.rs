use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PatientPlugin;

#[derive(Component)]
pub struct Patient;

/// This plugin handles patient related stuff like movement
/// Patient logic is only active during the State `GameState::Playing`
impl Plugin for PatientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_patient.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_patient.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_patient(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_hospital.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Patient);
}

fn move_patient(
    time: Res<Time>,
    actions: Res<Actions>,
    mut patient_query: Query<&mut Transform, With<Patient>>,
) {
    if actions.patient_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.patient_movement.unwrap().x * speed * time.delta_seconds(),
        actions.patient_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut patient_transform in &mut patient_query {
        patient_transform.translation += movement;
    }
}
