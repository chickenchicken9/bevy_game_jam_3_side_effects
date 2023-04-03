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
pub struct PillPlugin;

#[derive(Component)]
pub struct Pill;

pub struct SpawnPillEvent {
    pub pos: Vec3,
    pub dir: Quat,
}

impl Plugin for PillPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_pill.in_set(OnUpdate(GameState::Playing)))
            // .add_system(handle_mouse.in_set(OnUpdate(GameState::Playing)))
            .add_system(spawn_pills.in_set(OnUpdate(GameState::Playing)))
            .add_event::<SpawnPillEvent>();
    }
}

// fn handle_mouse(
//     mut mousebtn_evr: EventReader<MouseButtonInput>,
//     primary_window: Query<&Window, With<PrimaryWindow>>,
//     mut ev_spawn_pill: EventWriter<SpawnPillEvent>,
//     camera_q: Query<(&Camera, &GlobalTransform)>,
// ) {
//     let window = primary_window.single();
//     let (camera, camera_transform) = camera_q.single();

//     use bevy::input::ButtonState;

//     for ev in mousebtn_evr.iter() {
//         match ev.state {
//             ButtonState::Pressed => {
//                 println!("Mouse button press: {:?}", ev.button);
//             }
//             ButtonState::Released => {
//                 println!("Mouse button release: {:?}", ev.button);
//                 if let Some(pos) = window.cursor_position() {
//                     println!("Mouse released @ {:?}", pos);

//                     if let Some(world_position) = window
//                         .cursor_position()
//                         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//                         .map(|ray| ray.origin.truncate())
//                     {
//                         eprintln!("World coords: {}/{}", world_position.x, world_position.y);
//                         ev_spawn_pill.send(SpawnPillEvent(world_position));
//                     }
//                 }
//             }
//         }
//     }
// }

const pill_scale: f32 = 0.1;

pub fn spawn_pills(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut ev_spawn_pill: EventReader<SpawnPillEvent>,
    assets: Res<Assets<Image>>,
) {
    let mut rng = StdRng::from_entropy();
    let text_path = format!("textures/pill_{}.png", rng.next_u32() % 4);
    let text = textures.folder.get(&text_path).unwrap();
    let img = assets.get(text).unwrap();

    for ev in ev_spawn_pill.iter() {
        let mut points = Vec::new();

        for _ in 0..10 {
            let x: f32 = Standard.sample(&mut rng);
            let y: f32 = Standard.sample(&mut rng);
            points.push(Vect::new(
                x * img.size().x - img.size().x / 2.,
                y * img.size().y - img.size().y / 2.,
            ));
        }

        // random torque
        let torque_scale = 0.05;
        let torque_sample: f32 = Standard.sample(&mut rng);
        let torque_impulse = torque_sample * torque_scale - torque_scale / 2.;
        
        let force_scale = 300.;
        // random forces
        /*

        let x_force_sample: f32 = Standard.sample(&mut rng);
        let y_force_sample: f32 = Standard.sample(&mut rng);

        let x_force = x_force_sample * force_scale - force_scale / 2.;
        let y_force = y_force_sample * force_scale - force_scale / 2.;
        */

        // force based on beaker rotation
        let impulse: Vec2 = ev.dir.mul_vec3(Vec3::new(0., 1., 0.)).truncate() * force_scale;

        println!("ev.dir: {:?}, impulse: {:?}", ev.dir, impulse);

        commands
            .spawn(SpriteBundle {
                texture: text.clone(),
                transform: Transform::from_translation(Vec3::new(ev.pos.x, ev.pos.y, 1.))
                    .with_scale(Vec3::new(pill_scale, pill_scale, 1.)),
                ..Default::default()
            })
            .insert(Pill)
            .insert(RigidBody::Dynamic)
            // .insert(Collider::ball(60.0))
            .insert(Collider::convex_hull(&points).unwrap())
            .insert(ColliderMassProperties::Density(50.))
            .insert(Restitution::coefficient(0.9))
            .insert(ExternalImpulse {
                impulse,
                torque_impulse,
            });
    }
}

fn move_pill(
    time: Res<Time>,
    mut pill_query: Query<&mut Transform, With<Pill>>,
    mut ext_forces: Query<&mut ExternalForce>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
) {
    // for mut ext_force in ext_forces.iter_mut() {
    //     ext_force.force = Vec2::new(0., 0.0);
    //     ext_force.torque = 0.0;
    // }
}
