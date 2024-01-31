use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>().add_state::<InGameState>();
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Copy, Hash, Default)]
pub enum InGameState {
    #[default]
    Initial,
    Running,
    Paused,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Copy, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
