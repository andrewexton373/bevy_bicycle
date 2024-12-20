use bevy::prelude::*;

use crate::{bicycle::{chain::events::ResetChainEvent, events::SpawnBicycleEvent}, camera::events::{CameraZoomDirection, CameraZoomEvent}};

use super::plugin::UserInputPlugin;

impl UserInputPlugin {
    pub fn handle_user_input(
        mut commands: Commands,
        keys: Res<ButtonInput<KeyCode>>
    ) {
        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowUp => {commands.trigger(CameraZoomEvent(CameraZoomDirection::In));},
                KeyCode::ArrowDown => {commands.trigger(CameraZoomEvent(CameraZoomDirection::Out));},
                KeyCode::KeyR => {commands.trigger(ResetChainEvent);},
                KeyCode::Enter => {commands.trigger(SpawnBicycleEvent);},
                KeyCode::KeyQ => {}
                _ => {}
            }
        }
    }
}

