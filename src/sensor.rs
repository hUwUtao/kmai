use std::{collections::BTreeSet, default};

use crate::MainCamera;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    utils::HashMap,
};

use crate::config::{Sensor, SENSOR_MAPPING};

/// Individual sensor should act like any button event, except it is stateless.
#[derive(Event, Clone, Copy)]
pub struct SensorEvent {
    pub index: Sensor,
    pub mapping: &'static [f32; 5],
    pub is_down: bool,
}

fn rect_hit(position: Vec2, rotation: f32, scale: Vec2, hit_point: Vec2) -> bool {
    let tpoi = hit_point - position;
    let rpoi = Vec2::new(
        tpoi.x * rotation.cos() - tpoi.y * rotation.sin(),
        tpoi.x * rotation.sin() + tpoi.y * rotation.cos(),
    );
    let spoi = rpoi / scale;
    spoi.x.abs() <= 0.5 && spoi.y.abs() <= 0.5
}

fn hit_any(hit: Vec2) -> Option<u8> {
    for (index, sensor) in SENSOR_MAPPING.iter().enumerate() {
        if rect_hit(
            vec2(sensor[0], sensor[1]),
            sensor[4].to_radians(),
            vec2(sensor[2], sensor[3]),
            hit,
        ) {
            return Some(index as u8);
        }
    }
    None
}

impl SensorEvent {
    fn new(index: u8, is_down: bool) -> Option<Self> {
        Sensor::map_in(index as usize).map(|index| Self {
            index,
            is_down,
            mapping: &SENSOR_MAPPING[index as usize],
        })
    }
    pub fn send(&self, sender: &mut EventWriter<Self>) {
        sender.send(*self);
    }
}

struct SensorFrame(Vec<u8>);

#[derive(Resource)]
struct SensorFrameAll([SensorFrame; 2]);

impl Default for SensorFrameAll {
    fn default() -> Self {
        Self([
            SensorFrame(vec![0].repeat(34)),
            SensorFrame(vec![0].repeat(34)),
        ])
    }
}

impl SensorFrameAll {
    pub fn mix(&self) -> BTreeSet<u8> {
        let mut set: BTreeSet<u8> = BTreeSet::new();
        for s in &self.0 {
            let mut set_b = BTreeSet::from_iter(s.0.iter().map(|&s| s));
            set.append(&mut set_b);
        }
        set
    }
}

#[derive(Component)]
pub struct SensorReflect(pub Sensor);

pub struct SensorPlugin;
impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SensorFrameAll>()
            .init_resource::<TouchTrack>()
            .add_event::<SensorEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    touch_event,
                    // cursor_event,
                    kbd_event,
                ),
            )
            .add_systems(Update, draw_gizmo);
    }
}

fn vec_from_ad(a: f32, d: f32) -> Vec2 {
    vec2(-d * a.cos(), -d * a.sin())
}

fn setup(mut commands: Commands) {
    let hierachy = commands.spawn(SpatialBundle { ..default() }).id();
    for (i, (tf, s)) in SENSOR_MAPPING.iter().zip(Sensor::iter()).enumerate() {
        let sen_origin = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(
                        vec3(tf[0], tf[1], -1.0)
                            * Vec2::splat(match s.range() {
                                4 => 1.12,
                                _ => 1.0,
                            })
                            .extend(1.0),
                    )
                    .with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        0.0,
                        0.0,
                        tf[4],
                    )),
                    ..default()
                },
                SensorReflect(s.clone()),
            ))
            .id();

        commands.entity(hierachy).push_children(&[sen_origin]);
    }
}

// fn setup(mut windows: ResMut<Windows>) {
//     let window = windows.get_primary_mut().unwrap();
//     window.set_cursor_grab(true);
//     window.set_cursor_visible(false);
// }

// fn cursor_event(

//     q_window: Query<&Window, With<PrimaryWindow>>,
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) {
//     let (camera, camera_transform) = q_camera.single();
//     let window = q_window.single();
//     if let Some(world_position) = window
//         .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//         .map(|ray| ray.origin.truncate())
//     {
//         if Some(hit) = hit_any(world_position) {

//         }
//     }
// }

#[derive(Resource, Default)]
struct TouchTrack(HashMap<usize, u8>);

fn touch_event(
    mut sensor_evnt: EventWriter<SensorEvent>,
    mut hit: ResMut<SensorFrameAll>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    touches: Res<Touches>,
    mut ttrack: ResMut<TouchTrack>,
) {
    let (camera, camera_transform) = q_camera.single();

    hit.0[0].0 = touches
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            if let Some(vhit) = camera.viewport_to_world_2d(camera_transform, t.position()) {
                if let Some(lhit) = hit_any(vhit) {
                    // if let Some(last_sensor) = ttrack.0.get(&i) {
                    //     if *last_sensor != lhit {
                    //         sensor_evnt.send_batch(
                    //             [
                    //                 SensorEvent::new(lhit, true),
                    //                 SensorEvent::new(*last_sensor, false),
                    //             ]
                    //             .iter()
                    //             .filter_map(|x| *x),
                    //         );
                    //     }
                    // } else if let Some(s) = SensorEvent::new(lhit, false) {
                    //     ttrack.0.insert(i, lhit);
                    //     sensor_evnt.send(s);
                    // }
                    return Some(lhit);
                        
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()

    // for finger in touches.iter() {
    //     let fid = finger.id();
    //     if touches.just_released(fid) || touches.just_canceled(fid) {
    //         fingers.0.remove_entry(&fid);
    //         continue;
    //     }
    //     fingers.0.insert(
    //         fid,
    //         (camera.viewport_to_world_2d(camera_transform, finger.position())).unwrap_or_default(),
    //     );
    // }

    // fingers.touch_update(
    // touches
    //         .iter()
    //         .map(|touch| {
    //             if let Some(touch) = touches.get_pressed(touch.id()) {
    //                 Some(
    //                     (camera.viewport_to_world_2d(camera_transform, touch.position()))
    //                         .unwrap_or_default(),
    //                 )
    //             } else {
    //                 None
    //             }
    //         })
    //         .filter(|opt| opt.is_some())
    //         .map(|somes| somes.unwrap_or_default())
    //         .collect(),
    // )
    // use bevy::input::touch::TouchPhase;
    // for ev in touch_evr.read() {
    //     // in real apps you probably want to store and track touch ids somewhere
    //     if let Some(hit) = camera.viewport_to_world_2d(camera_transform, ev.position) {
    //         match ev.phase {
    //             TouchPhase::Started | TouchPhase::Moved => {
    //                 if let (Some(ray), Some(last_id)) =
    //                     (SensorEvent::hit_any(hit), tracker.0.get(&ev.id))
    //                 {
    //                     if ray != *last_id {
    //                         sensor_evnt.send(SensorEvent::new(*last_id, false));
    //                     }
    //                     tracker.0.insert(ev.id, ray);
    //                     sensor_evnt.send(SensorEvent::new(ray, true));
    //                 };
    //             }
    //             TouchPhase::Ended | TouchPhase::Canceled => {
    //                 if let Some(ray) = SensorEvent::hit_any(ev.position) {
    //                     sensor_evnt.send(SensorEvent::new(ray, false));
    //                     tracker.0.remove(&ev.id);
    //                 }
    //             }
    //         }
    //     }
    // }
}

fn kbd_event(
    mut sensor_evnt: EventWriter<SensorEvent>,
    mut hit: ResMut<SensorFrameAll>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    hit.0[1].0 = [
        KeyCode::KeyW,
        KeyCode::KeyE,
        KeyCode::KeyD,
        KeyCode::KeyC,
        KeyCode::KeyX,
        KeyCode::KeyZ,
        KeyCode::KeyA,
        KeyCode::KeyQ,
    ]
    .iter()
    .enumerate()
    .filter_map(|(i, ck)| {
        let assigned_sensor = i + 26;

        if kbd.just_pressed(*ck) {
            (SensorEvent::new(assigned_sensor as u8, true)
                .unwrap()
                .send(&mut sensor_evnt));
        }
        if kbd.just_released(*ck) {
            (SensorEvent::new(assigned_sensor as u8, false)
                .unwrap()
                .send(&mut sensor_evnt));
        }

        if kbd.pressed(*ck) {
            Some(assigned_sensor as u8)
        } else {
            None
        }
    })
    .collect()
}

fn draw_gizmo(mut gizmos: Gizmos, hit_frame: Res<SensorFrameAll>) {
    for (index, sensor) in SENSOR_MAPPING.iter().enumerate() {
        let pos = Vec2 {
            x: sensor[0],
            y: sensor[1],
        };
        let rot = sensor[4].to_radians();
        let scl = Vec2 {
            x: sensor[2],
            y: sensor[3] + 0.5,
        };

        let hit = hit_frame.mix().contains(&(index as u8));

        // let hit = (move |managed_touches: &ManagedTouches| {
        //     for t in managed_touches.digest() {
        //         if rect_hit(pos, rot, scl, t) {
        //             return true;
        //         }
        //     }
        //     false
        // })(&(managed_touches));

        // if !hit {
        //     continue;
        // }
        gizmos.rect_2d(
            pos,
            rot,
            scl,
            if hit {
                Color::rgb(1.0, 1.0, 1.0)
            } else {
                Color::rgba(0.5, 0.5, 0.5, 0.2)
            },
        );
    }
    gizmos.circle_2d(Vec2::ZERO, 540.0, Color::rgb(1.0, 1.0, 1.0));
}
