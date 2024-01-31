use crate::state::{AppState, InGameState};
use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_game.run_if(in_state(AppState::MainMenu)),
                pause_game.run_if(in_state(InGameState::Running)),
                resume_game.run_if(in_state(InGameState::Paused)),
            ),
        );
    }
}

fn start_game(
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Z) {
        next_app_state.set(AppState::InGame);
        next_in_game_state.set(InGameState::Running);
    }
}

fn pause_game(
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_in_game_state.set(InGameState::Paused);
    }
}

fn resume_game(
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_in_game_state.set(InGameState::Running);
    }
}
