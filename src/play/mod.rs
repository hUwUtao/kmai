use bevy_kira_audio::prelude::*;
use bevy::{prelude::*, transform::commands};

use crate::GameState;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Smash), setup);
    }
}

fn setup() {}

#[derive(Bundle)]
struct Note {}


fn debug_spawn_note(mut commands: Commands, kbd: Res<ButtonInput<KeyCode>>) {

}