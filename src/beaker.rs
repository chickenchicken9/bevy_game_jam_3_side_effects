use crate::loading::TextureAssets;
use crate::GameState;
use bevy::input::mouse::MouseButtonInput;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy::window::Window;

use rand::distributions::{Distribution};



use crate::pill::SpawnPillEvent;

#[derive(Component)]
pub struct Beaker;

pub struct BeakerPlugin;

impl Plugin for BeakerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_beakers.in_schedule(OnEnter(GameState::Playing)))
            .add_system(handle_beaker_touch.in_set(OnUpdate(GameState::Playing)))
            .add_system(handle_beaker_hover.in_set(OnUpdate(GameState::Playing)));
    }
}

const beaker_scale: f32 = 0.4;
const beaker_click_dist: f32 = 100.;

fn spawn_beakers(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let text = textures.folder.get("textures/beaker.png").unwrap();
    let window = window_q.single();

    let margin: f32 = 30.;
    let x: f32 = window.resolution.width() / 2. - margin;
    let y: f32 = window.resolution.height() / 2. - margin;
    let _z: f32 = 1.;
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
            .spawn(Beaker)
            .insert(Interaction::Clicked)
            .insert(SpriteBundle {
                texture: text.clone(),
                transform: Transform::from_xyz(pos.0, pos.1, pos.2)
                    .with_scale(Vec3::new(beaker_scale, beaker_scale, 1.))
                    .with_rotation(Quat::from_rotation_z((pos.3).to_radians())),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(beaker_click_dist / beaker_scale).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    // transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                });
            });
    }
}

fn handle_beaker_hover(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    textures: Res<TextureAssets>,
    mut beakers: Query<(&GlobalTransform, &mut Handle<Image>), With<Beaker>>,
) {
    let window = primary_window.single();
    let (camera, camera_transform) = camera_q.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for (transform, mut texture) in beakers.iter_mut() {
            if transform.translation().truncate().distance(world_position) < beaker_click_dist {
                *texture = textures
                    .folder
                    .get("textures/beaker_hover.png")
                    .unwrap()
                    .clone();
            } else {
                *texture = textures.folder.get("textures/beaker.png").unwrap().clone();
            }
        }
    }
}

fn handle_beaker_touch(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut ev_spawn_pill: EventWriter<SpawnPillEvent>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    beakers: Query<&GlobalTransform, With<Beaker>>,
) {
    let window = primary_window.single();
    let (camera, camera_transform) = camera_q.single();

    use bevy::input::ButtonState;
    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                // println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                // println!("Mouse button release: {:?}", ev.button);
                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    for b in beakers.iter() {
                        if b.translation().truncate().distance(world_position) < beaker_click_dist {
                            let (_scale, dir, pos) = b.to_scale_rotation_translation();
                            ev_spawn_pill.send(SpawnPillEvent { pos, dir });
                        }
                    }
                }
            }
        }
    }
}
