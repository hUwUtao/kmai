#![allow(clippy::type_complexity)]

use bevy::app::App;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;

use bevy::prelude::*;

mod play;
mod sensor;
mod view;

mod effect;

pub mod config;

use sensor::SensorPlugin;
pub use view::{MainCamera, ViewPlugin};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[allow(dead_code)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Transition,

    PadServer,

    SongSelect,

    #[default]
    Smash,

    Settings,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((ViewPlugin, SensorPlugin, effect::EffectPlugin));

        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            #[cfg(debug_assertions)]
            {
                LogDiagnosticsPlugin::default()
            },
        ));
    }
}
