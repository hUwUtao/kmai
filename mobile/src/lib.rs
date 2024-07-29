use bevy::prelude::*;
use bevy::window::WindowMode;
use kanimai::config::config_window;
use kanimai::GamePlugin; // ToDo: Replace bevy_game with your new crate name.

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    present_mode: bevy::window::PresentMode::Fifo,
                    ..config_window()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}
