use bevy::prelude::*;
// use bevy_kira_audio::prelude::*;

use crate::GameState;

pub mod lanes;
pub mod timeline;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(lanes::TapNotePlugin)
            .add_systems(OnEnter(GameState::Smash), setup);
    }
}

fn setup() {}
