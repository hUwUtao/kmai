use bevy::{ecs::query, log, prelude::*, time::Stopwatch, transform::commands};

#[derive(Component, Clone, Copy)]
pub struct ParticleSize {
    start: f32,
    end: f32,
}

#[derive(Component, Clone, Copy)]
pub struct ParticleVelocity {
    start: Vec2,
    end: Vec2,
}

#[derive(Component, Clone, Copy)]
pub struct ParticleColor {
    start: Color,
    end: Color,
}

#[derive(Component)]
pub struct Particle {
    lifetime: Timer,
}

#[derive(Component)]
pub struct ParticleSpawnerTimer(Timer);

use crate::{
    config::Sensor,
    sensor::{SensorEvent, SensorReflect},
};

pub mod interp;

use self::interp::*;
// use bevy_kira_audio::prelude::*;
// use std::collections::BTreeMap;

// use crate::GameState;

// pub struct HanabiEventPayload {
//     hanabi: Handle<EffectAsset>,
// }

// #[derive(PartialEq, Eq, PartialOrd, Ord)]
// pub enum DefinedHanabi {
//     Tap,
// }

// #[derive(Resource, Default)]
// pub struct HanabiStore(BTreeMap<DefinedHanabi, HanabiEventPayload>);

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(HanabiPlugin)
            // .init_resource::<HanabiStore>()
            .add_systems(PostStartup, setup)
            .add_systems(PreUpdate, tick_particle)
            .add_systems(PostUpdate, (dispatch_hit, debug));
    }
}

fn setup(
    mut commands: Commands,
    query: Query<Entity, With<SensorReflect>>,
    // trig: Res<ButtonInput<KeyCode>>,
    asv: Res<AssetServer>,
) {
    // commands.spawn(ParticleEffectBundle {
    //     effect: ParticleEffect::new(effect_handle),
    //     ..default()
    // });
    // efpack.0.insert(
    //     DefinedHanabi::Tap,
    //     HanabiEventPayload {
    //         hanabi: effect_handle,
    //     },
    // );
    // for i in query.iter() {
    //     spawn_particle(
    //         &mut commands,
    //         &i,
    //         ParticleConfiguration {
    //             amount: 8,
    //             behaviour: ParticleBehaviour::ShootEqual,
    //             lifespan: 10.0,
    //             sprite: asv.load("textures/github.png"),
    //         },
    //     )
    // }
}

#[derive(Component)]
pub struct ParticleMember(u8, ParticleConfiguration, Stopwatch);

#[derive(Clone)]
pub enum ParticleBehaviour {
    Tap,
}

fn whatever_tap_but_cool(
    transform: &mut Transform,
    render: &mut Color,
    index: u8,
    pool_size: u8,
    elapsed_secs: f32,
    lifespan_secs: f32,
) {
    let progress = ease_in_out_quad(elapsed_secs / lifespan_secs);

    let angle = index as f32 / pool_size as f32 * 2.0 * std::f32::consts::PI;
    let distance = progress * if (index & 0x1) != 0 { 32.0 } else { 40.0 };

    // (
    //     Vec2::new(distance * angle.cos(), distance * angle.sin()),
    //     ease_in_out_quad(1.0 - progress),
    // )
    transform.translation = Vec2::new(distance * angle.cos(), distance * angle.sin()).extend(-1.0);
    transform.rotate_z(10.0);
    transform.scale =
        Vec3::splat((0.5 + (progress * 0.5)) * if (index & 0x1) != 0 { 0.3 } else { 0.1 });
    render.set_a(1.0 - progress);
}

#[derive(Clone)]
pub struct ParticleConfiguration {
    amount: u8,
    behaviour: ParticleBehaviour,
    /// in sec
    lifespan: f32,
    sprite: Handle<Image>,
}

fn spawn_particle(commands: &mut Commands, root: &Entity, config: ParticleConfiguration) {
    for i in 0..config.amount.clone() {
        let c = commands
            .spawn((
                SpriteBundle {
                    texture: config.sprite.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.25)),
                    ..default()
                },
                ParticleMember(i, config.clone(), Stopwatch::new()),
            ))
            .id();
        commands.entity(*root).push_children(&[c]);
    }
}

fn tick_particle(
    time: Res<Time>,
    mut commands: Commands,
    mut particles: Query<(Entity, &mut ParticleMember, &mut Transform, &mut Sprite)>,
) {
    for (ent, mut pinf, mut tf, mut sprite) in particles.iter_mut() {
        pinf.2.tick(time.delta());
        if pinf.1.lifespan < pinf.2.elapsed_secs() {
            commands.entity(ent).despawn();
            continue;
        };
        // info!("{}", pinf.2.elapsed_secs());
        (match pinf.1.behaviour {
            ParticleBehaviour::Tap => whatever_tap_but_cool,
        }(
            &mut tf,
            &mut sprite.color,
            pinf.0,
            pinf.1.amount,
            pinf.2.elapsed_secs(),
            pinf.1.lifespan,
        ));

        // *tf = tf.with_translation(tf2.extend(-1.0));
        // tf.translation = tf2.extend(-1.0);
        // sprite.color.set_a(op);
    }
}

fn dispatch_hit(
    mut commands: Commands,
    mut evnt: EventReader<SensorEvent>,
    query: Query<(Entity, &SensorReflect)>,
    asv: Res<AssetServer>,
) {
    for ev in evnt.read() {
        if ev.is_down {
            if let Some((e, _r)) = query.iter().find(move |(_e, &ref s)| s.0 == ev.index) {
                spawn_particle(
                    &mut commands,
                    &e,
                    ParticleConfiguration {
                        amount: 8,
                        behaviour: ParticleBehaviour::Tap,
                        lifespan: 0.25,
                        sprite: asv.load("textures/hit/FX_hy_18_020.png"),
                    },
                )
            }
        }
    }
}

fn debug(
    mut commands: Commands,
    query: Query<Entity, With<SensorReflect>>,
    trig: Res<ButtonInput<KeyCode>>,
    asv: Res<AssetServer>,
) {
    if trig.just_pressed(KeyCode::KeyP) {
        // info!("attempt to spam sht");
        // for (i, p) in res.0.iter() {
        // commands.spawn(ParticleEffectBundle {
        //     effect: ParticleEffect::new(p.hanabi.clone_weak()),
        //     ..default()
        // });
        // }
        // for mut ent in query.iter_mut() {
        //     ent.reset();
        // }
        for i in query.iter() {
            spawn_particle(
                &mut commands,
                &i,
                ParticleConfiguration {
                    amount: 8,
                    behaviour: ParticleBehaviour::Tap,
                    lifespan: 0.5,
                    sprite: asv.load("textures/hit/FX_hy_18_020.png"),
                },
            )
        }
    }
}
