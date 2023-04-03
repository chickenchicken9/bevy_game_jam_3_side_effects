use crate::loading::FontAssets;

use crate::patient::PatientHealedEvent;
use crate::GameState;
use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_ui.in_set(OnUpdate(GameState::Playing)))
            .add_system(cleanup_ui.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component)]
struct UiEntity;

#[derive(Component)]
struct PatientTracker {
    saved: i32,
}

fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(UiEntity)
        .insert(PatientTracker { saved: 0 })
        .insert(
            TextBundle::from_section(
                "Patients saved: 0",
                TextStyle {
                    font: font_assets.fira_sans.clone(),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            )
            .with_text_alignment(TextAlignment::Left)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(15.0),
                    left: Val::Px(100.0),
                    ..default()
                },
                ..default()
            }),
        );
        
    commands
    .spawn(UiEntity)
    .insert(
        TextBundle::from_section(
            "Tap the beakers to pop out pills.\nSave the patients!",
            TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 20.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(15.0),
                right: Val::Px(100.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn update_ui(
    mut query: Query<(&mut PatientTracker, &mut Text)>,
    mut events: EventReader<PatientHealedEvent>,
) {
    let (mut tracker, mut text) = query.single_mut();
    let mut changed = false;
    for _ev in events.iter() {
        tracker.saved += 1;
        changed = true;
    }

    if changed {
        text.sections[0].value = format!("Patients saved: {}", tracker.saved);
    }
}

fn cleanup_ui(mut commands: Commands, entities: Query<Entity, With<UiEntity>>) {
    for e in entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}
