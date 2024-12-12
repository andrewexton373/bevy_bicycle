use bevy::prelude::*;

use super::{
    chain::plugin::ChainPlugin, events::SpawnBicycleEvent, groupset::plugin::GroupsetPlugin,
    wheel::plugin::WheelPlugin,
};

pub struct BicyclePlugin;

impl Plugin for BicyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChainPlugin,
            // SprocketPlugin,
            GroupsetPlugin,
            WheelPlugin,
        ))
        .add_systems(Startup, BicyclePlugin::init_bicycle)
        .add_observer(BicyclePlugin::spawn_frame)
        // .add_observer(BicyclePlugin::spawn_crank)
        .add_event::<SpawnBicycleEvent>();
        // .add_event::<SpawnCrankEvent>();
    }
}
