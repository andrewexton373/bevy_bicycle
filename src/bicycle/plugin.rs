use bevy::prelude::*;

use super::{chain::plugin::ChainPlugin, groupset::plugin::GroupsetPlugin, sprocket::plugin::SprocketPlugin};

pub struct BicyclePlugin;

impl Plugin for BicyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChainPlugin,
            // SprocketPlugin, 
            GroupsetPlugin
        ));
            // .add_systems(Startup, BicyclePlugin::setup_bicycle);
            // .add_systems(Update,
            //     (
            //         // BicyclePlugin::spin_wheel
            //     ));
    }
}
