use crate::state::{AppState, InGameState};
use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            main_menu_controls.run_if(in_state(AppState::MainMenu)),
        );
    }
}

fn main_menu_controls(
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Z) {
        next_app_state.set(AppState::InGame);
        next_in_game_state.set(InGameState::Running);
    }
}
