use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy_rapier2d::prelude::*;

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
    for ev in ev_spawn_pill.iter() {
        commands
            .spawn(SpriteBundle {
                texture: textures.folder.get("textures/pill_0.png").unwrap().clone(),
                transform: Transform::from_translation(Vec3::new(ev.0.x, ev.0.y, 1.)),
                ..Default::default()
            })
            .insert(Pill)
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(60.0))
            .insert(ColliderMassProperties::Density(2.0))
            .insert(Restitution::coefficient(0.7))
            .insert(ExternalForce {
                force: Vec2::new(100.0, 200.0),
                torque: 14.0,
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
