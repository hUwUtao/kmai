use std::time::Duration;

use bevy::{
    math::{u32, vec3},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

/**
 * This file contains both Tap Lane and Tap Note behaviour
 */
use crate::config::Sensor;

pub enum OptionNotespeedId {
    Speed1_0,
    Speed1_25,
    Speed1_5,
    Speed1_75,
    Speed2_0,
    Speed2_25,
    Speed2_5,
    Speed2_75,
    Speed3_0,
    Speed3_25,
    Speed3_5,
    Speed3_75,
    Speed4_0,
    Speed4_25,
    Speed4_5,
    Speed4_75,
    Speed5_0,
    Speed5_25,
    Speed5_5,
    Speed5_75,
    Speed6_0,
    Speed6_25,
    Speed6_5,
    Speed6_75,
    Speed7_0,
    Speed7_25,
    Speed7_5,
    Speed7_75,
    Speed8_0,
    Speed8_25,
    Speed8_5,
    Speed8_75,
    Speed9_0,
    Speed9_25,
    Speed9_5,
    Speed9_75,
    Speed10_0,
    SpeedSonic,
}

impl OptionNotespeedId {
    pub fn get_value(&self) -> f32 {
        match *self {
            OptionNotespeedId::Speed1_0 => 60.0,
            OptionNotespeedId::Speed1_25 => 75.0,
            OptionNotespeedId::Speed1_5 => 90.0,
            OptionNotespeedId::Speed1_75 => 105.0,
            OptionNotespeedId::Speed2_0 => 120.0,
            OptionNotespeedId::Speed2_25 => 150.0,
            OptionNotespeedId::Speed2_5 => 180.0,
            OptionNotespeedId::Speed2_75 => 210.0,
            OptionNotespeedId::Speed3_0 => 240.0,
            OptionNotespeedId::Speed3_25 => 270.0,
            OptionNotespeedId::Speed3_5 => 300.0,
            OptionNotespeedId::Speed3_75 => 330.0,
            OptionNotespeedId::Speed4_0 => 360.0,
            OptionNotespeedId::Speed4_25 => 390.0,
            OptionNotespeedId::Speed4_5 => 420.0,
            OptionNotespeedId::Speed4_75 => 450.0,
            OptionNotespeedId::Speed5_0 => 480.0,
            OptionNotespeedId::Speed5_25 => 510.0,
            OptionNotespeedId::Speed5_5 => 540.0,
            OptionNotespeedId::Speed5_75 => 570.0,
            OptionNotespeedId::Speed6_0 => 600.0,
            OptionNotespeedId::Speed6_25 => 630.0,
            OptionNotespeedId::Speed6_5 => 660.0,
            OptionNotespeedId::Speed6_75 => 690.0,
            OptionNotespeedId::Speed7_0 => 720.0,
            OptionNotespeedId::Speed7_25 => 750.0,
            OptionNotespeedId::Speed7_5 => 780.0,
            OptionNotespeedId::Speed7_75 => 810.0,
            OptionNotespeedId::Speed8_0 => 840.0,
            OptionNotespeedId::Speed8_25 => 870.0,
            OptionNotespeedId::Speed8_5 => 900.0,
            OptionNotespeedId::Speed8_75 => 930.0,
            OptionNotespeedId::Speed9_0 => 960.0,
            OptionNotespeedId::Speed9_25 => 990.0,
            OptionNotespeedId::Speed9_5 => 1020.0,
            OptionNotespeedId::Speed9_75 => 1050.0,
            OptionNotespeedId::Speed10_0 => 1080.0,
            OptionNotespeedId::SpeedSonic => 1080.0,
        }
    }
}

struct LaneReflect(pub Sensor);

pub struct FlowData {
    low: f32,
    high: f32,
    time: OptionNotespeedId,
}

pub struct LaneData {
    lane_deg: f32,
    reflect: LaneReflect,
}

fn lane_ang_calc(i: u8) -> f32 {
    22.5 + (i * 45) as f32
}

fn lane_dxspd_calc(v: f32) -> f32 {
    1.0
}

#[derive(Resource)]
pub struct ActiveNotes {}

#[derive(Component)]
pub struct NoteRender;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct NoteRenderMaterial {
    #[texture(0)]
    #[sampler(1)]
    atlas: Option<Handle<Image>>,
    #[uniform(2)]
    offset: u32,
    #[uniform(3)]
    ttr: u32,
}

impl Material2d for NoteRenderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tap.wgsl".into()
    }
}

#[derive(Component)]
pub struct NoteTimer(Timer);

pub(crate) struct TapNotePlugin;
impl Plugin for TapNotePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<NoteRenderMaterial>::default())
            .add_systems(Startup, Self::setup)
            .add_systems(Update, Self::update);
    }
}
impl TapNotePlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<NoteRenderMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default()
                    .with_translation(vec3(0.0, 0.0, -9.0))
                    .with_scale(Vec3::splat(1080.)),
                material: materials.add(NoteRenderMaterial {
                    atlas: Some(asset_server.load("textures/hit/atlas_repacked.png")),
                    offset: 0,
                    ttr: 690,
                }),
                ..default()
            },
            NoteTimer(Timer::new(Duration::from_secs(10 * 60), TimerMode::Once)),
        ));
    }
    fn update(
        mut q: Query<(&mut NoteTimer, &Handle<NoteRenderMaterial>)>,
        mut materials: ResMut<Assets<NoteRenderMaterial>>,
        time: Res<Time>,
    ) {
        if let Ok((mut timer, material)) = q.get_single_mut() {
            timer.0.tick(time.delta());
            materials.get_mut(material).unwrap().offset = timer.0.elapsed().as_millis() as u32
        }
    }
}
