use bevy::prelude::*;

use crate::GameState;
pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            UserInputPlugin::handle_user_input.run_if(in_state(GameState::Ready)),
        );
    }
}
