use crate::loading::TextureAssets;
use crate::GameState;
use bevy::input::mouse::MouseButtonInput;

use bevy::input::touch::TouchInput;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy::window::Window;

use crate::pill::SpawnPillEvent;

#[derive(Component)]
pub struct Beaker;

pub struct BeakerPlugin;

struct TapEvent(Vec2);

impl Plugin for BeakerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_beakers.in_schedule(OnEnter(GameState::Playing)))
            .add_system(handle_taps.in_set(OnUpdate(GameState::Playing)))
            .add_system(handle_clicks_and_touches.in_set(OnUpdate(GameState::Playing)))
            .add_system(handle_beaker_hover.in_set(OnUpdate(GameState::Playing)))
            .add_event::<TapEvent>();
    }
}

const BEAKER_SCALE: f32 = 0.4;
const BEAKER_CLICK_DIST: f32 = 100.;

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
    let z: f32 = 2.;
    let r: f32 = 30.;

    let pos = vec![
        // Left side, bottom-to-top
        (-x, -y, z, -r / 2.),
        (-x, 0., z, -r),
        (-x, y, z, -r * 2.),
        // Right side, bottom-to-top
        (x, -y, z, r / 2.),
        (x, 0., z, r),
        (x, y, z, r * 2.),
    ];

    for pos in pos {
        let mut transform = Transform::from_xyz(pos.0, pos.1, pos.2)
            .with_scale(Vec3::new(BEAKER_SCALE, BEAKER_SCALE, 1.))
            .with_rotation(Quat::from_rotation_z((pos.3).to_radians()));
        commands.spawn(Beaker).insert(SpriteBundle {
            texture: text.clone(),
            transform,
            ..Default::default()
        });

        transform.translation.z = 0.9;
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(BEAKER_CLICK_DIST / BEAKER_SCALE).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform,
            ..default()
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
            if transform.translation().truncate().distance(world_position) < BEAKER_CLICK_DIST {
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

fn handle_clicks_and_touches(
    mut touch_evr: EventReader<TouchInput>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ev_taps: EventWriter<TapEvent>,
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
                    ev_taps.send(TapEvent(world_position));
                }
            }
        }
    }

    for touch in touch_evr.iter() {
        match touch.phase {
            bevy::input::touch::TouchPhase::Started => {
                // ev_taps.send(TapEvent(touch.position));
                info!("Tap @ {:?}", touch.position);
                info!("Window pos: {:?}, size: {:?}", window.position, (window.width(), window.height()));

                // do touches have to be translated into world position like mouse clicks?

                if let Some(mut world_position) = camera
                    .viewport_to_world(camera_transform, touch.position)
                    .map(|ray| ray.origin.truncate())
                {
                    info!("Tap @ world_position {:?}", world_position);

                    // why is it flipped???
                    world_position.y = -world_position.y;

                    ev_taps.send(TapEvent(world_position));
                }
            }
            _ => {}
        }
    }
}

fn handle_taps(
    beakers: Query<&GlobalTransform, With<Beaker>>,
    mut taps: EventReader<TapEvent>,
    mut ev_spawn_pill: EventWriter<SpawnPillEvent>,
) {
    for tap in taps.iter() {
        for b in beakers.iter() {
            if b.translation().truncate().distance(tap.0) < BEAKER_CLICK_DIST {
                let (_scale, dir, pos) = b.to_scale_rotation_translation();
                ev_spawn_pill.send(SpawnPillEvent { pos, dir });
            }
        }
    }
}
