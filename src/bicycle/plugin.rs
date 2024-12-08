use bevy::prelude::*;

use super::{chain::plugin::ChainPlugin, groupset::plugin::GroupsetPlugin, sprocket::plugin::SprocketPlugin, systems::{SpawnBicycleEvent, SpawnCrankEvent, SpawnFrameEvent, SpawnWheelEvent}};

pub struct BicyclePlugin;

impl Plugin for BicyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChainPlugin,
            // SprocketPlugin, 
            GroupsetPlugin
        ))
            .add_systems(Startup, BicyclePlugin::initialize)
            .add_systems(Update, (
                BicyclePlugin::spawn_bicycle,
            ))
            .add_observer(BicyclePlugin::spawn_frame)
            .add_observer(BicyclePlugin::spawn_wheel)
            .add_observer(BicyclePlugin::spawn_crank)
            .add_event::<SpawnBicycleEvent>()
            // .add_event::<SpawnFrameEvent>()
            .add_event::<SpawnWheelEvent>()
            .add_event::<SpawnCrankEvent>();
    }
}
