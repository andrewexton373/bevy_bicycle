use bevy::prelude::*;

use crate::GameState;

use super::{
    chain::plugin::ChainPlugin,
    events::SpawnBicycleEvent,
    groupset::plugin::GroupsetPlugin,
    systems::{initialize, BicycleSystems},
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
        // .add_systems(Startup, initialize)
        // .add_systems(
        //     Update,
        //     BicyclePlugin::spawn_bicycle_on_startup.run_if(in_state(GameState::Ready)),
        // )
        .add_observer(BicyclePlugin::spawn_frame)
        .add_observer(BicyclePlugin::on_remove_bicyle)
        .add_observer(BicyclePlugin::init_bicycle)
        // .add_observer(BicyclePlugin::spawn_crank)
        .init_resource::<BicycleSystems>()
        .add_event::<SpawnBicycleEvent>();

        // .add_event::<SpawnCrankEvent>();
    }
}
