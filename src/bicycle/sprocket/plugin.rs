use bevy::prelude::*;

use super::components::SprocketInfo;

pub struct SprocketPlugin;

impl Plugin for SprocketPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SprocketInfo>()
            .add_systems(Startup, SprocketPlugin::setup_sproket);
    }
}
