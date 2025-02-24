// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;

#[cfg(debug_assertions)]
#[cfg(feature = "dev")]
use bevy_editor_pls::prelude::*;

use kanimai::config::config_window;
use kanimai::GamePlugin;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                #[cfg(target_os = "windows")]
                mode: bevy::window::WindowMode::Fullscreen,
                #[cfg(target_family = "wasm")]
                canvas: Some("#bevy".to_owned()),
                #[cfg(target_family = "wasm")]
                prevent_default_event_handling: false,
                ..config_window()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_systems(Startup, set_window_icon);

    #[cfg(target_os = "windows")]
    app.add_systems(Update, toggle_window_mode);

    #[cfg(debug_assertions)]
    #[cfg(feature = "dev")]
    app.add_plugins(EditorPlugin::default());

    app.run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
#[cfg(target_os = "windows")]
fn toggle_window_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if (keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight))
        && keyboard.just_pressed(KeyCode::Enter)
    {
        if let Ok(mut window) = window_query.get_single_mut() {
            window.mode = match window.mode {
                bevy::window::WindowMode::Windowed => bevy::window::WindowMode::Fullscreen,
                _ => bevy::window::WindowMode::Windowed,
            };
        }
    }
}
