use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    math::vec3,
    prelude::*,
    reflect::TypeData,
    render::camera::ScalingMode,
    sprite::Anchor,
};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct DiagnosticText;

pub struct ViewPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 1.05,
                scaling_mode: ScalingMode::AutoMin {
                    min_width: 1080.0,
                    min_height: 1080.0,
                },
                ..default()
            },
            // camera: Camera {
            //     viewport: Some(Viewport {
            //         // physical_position:
            //         ..default() }),
            //     ..default()
            // },
            ..default()
        },
        MainCamera,
    ));
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(vec3(0.0, 0.0, -10.0)),
        texture: asset_server.load("textures/outlines/Outline_03.png").into(),
        ..default()
    });
    let style = TextSection::from_style(TextStyle {
        font: asset_server.load("fonts/NotoSansMono-Medium.ttf"),
        font_size: 24.,
        color: Color::WHITE,
        ..default()
    });
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([style.clone(), style.clone()]),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_translation(Vec3::new(540.0, -540.0, 0.0)),
            ..default()
        },
        // }.with_style(Style {
        //     position_type: PositionType::Absolute,
        //     bottom: Val::Px(5.0),
        //     right: Val::Px(5.0),
        //     ..default()
        // }),
        DiagnosticText,
    ));
}

fn update(mut text: Query<&mut Text, With<DiagnosticText>>, diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            // Update the value of the second section
            text.get_single_mut().unwrap().sections[0].value = format!(
                "{value:.2} | {} build",
                if cfg!(debug_assertions) {
                    "debug"
                } else {
                    "release"
                }
            );
            // text.get_single_mut().unwrap().sections[1].value = format!("{value:.2}");
        }
    }
}
