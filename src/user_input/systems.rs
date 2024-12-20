use bevy::prelude::*;

use crate::{bicycle::{chain::events::ResetChainEvent, events::SpawnBicycleEvent}, camera::events::{CameraPanDirection, CameraPanEvent, CameraZoomDirection, CameraZoomEvent, CycleCameraModeEvent}};

use super::plugin::UserInputPlugin;

impl UserInputPlugin {
    pub fn handle_user_input(
        mut commands: Commands,
        keys: Res<ButtonInput<KeyCode>>
    ) {

        // Continuous Input
        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowUp => {commands.send_event(CameraZoomEvent(CameraZoomDirection::In));},
                KeyCode::ArrowDown => {commands.send_event(CameraZoomEvent(CameraZoomDirection::Out));},
                _ => {}
            }
        }

        // One Keystroke Input
        for key in keys.get_just_pressed() {
            match key {
                KeyCode::KeyW => {commands.send_event(CameraPanEvent(CameraPanDirection::Up));},
                KeyCode::KeyA => {commands.send_event(CameraPanEvent(CameraPanDirection::Left));},
                KeyCode::KeyS => {commands.send_event(CameraPanEvent(CameraPanDirection::Down));},
                KeyCode::KeyD => {commands.send_event(CameraPanEvent(CameraPanDirection::Right));},
                KeyCode::KeyC => {commands.send_event(CycleCameraModeEvent);}
                KeyCode::KeyR => {commands.trigger(ResetChainEvent);},
                KeyCode::Enter => {commands.trigger(SpawnBicycleEvent);},
                KeyCode::KeyQ => {}
                _ => {}
            }
        }
    }
}

