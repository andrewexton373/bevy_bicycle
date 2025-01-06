use std::alloc::System;

use bevy::prelude::*;

use crate::{
    bicycle::systems::BicycleSystems,
    camera::events::{
        CameraPanDirection, CameraPanEvent, CameraZoomDirection, CameraZoomEvent,
        CycleCameraModeEvent,
    },
};

use super::plugin::UserInputPlugin;

impl UserInputPlugin {
    pub fn handle_user_input(
        mut commands: Commands,
        systems: Res<BicycleSystems>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        // Continuous Input
        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowUp => {
                    commands.send_event(CameraZoomEvent(CameraZoomDirection::In));
                }
                KeyCode::ArrowDown => {
                    commands.send_event(CameraZoomEvent(CameraZoomDirection::Out));
                }
                KeyCode::KeyW => {
                    commands.send_event(CameraPanEvent(CameraPanDirection::Up));
                }
                KeyCode::KeyA => {
                    commands.send_event(CameraPanEvent(CameraPanDirection::Left));
                }
                KeyCode::KeyS => {
                    commands.send_event(CameraPanEvent(CameraPanDirection::Down));
                }
                KeyCode::KeyD => {
                    commands.send_event(CameraPanEvent(CameraPanDirection::Right));
                }
                _ => {}
            }
        }

        // One Keystroke Input
        for key in keys.get_just_pressed() {
            match key {
                KeyCode::KeyC => {
                    commands.send_event(CycleCameraModeEvent);
                }
                KeyCode::KeyR => {
                    // commands.trigger(ResetChainEvent);
                    // commands.run_system_once(ChainPlugin::spawn_chain);
                    commands.run_system(systems.0["spawn_chain"]);
                }
                KeyCode::Enter => {
                    commands.run_system(systems.0["spawn_bicycle"]);
                }
                KeyCode::KeyQ => {}
                _ => {}
            }
        }
    }
}
