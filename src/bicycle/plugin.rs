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
        .add_systems(Startup, BicyclePlugin::spawn_bicycle_on_startup)
        .add_systems(Update, BicyclePlugin::handle_reset_bicycle_input)
        .add_observer(BicyclePlugin::spawn_frame)
        .add_observer(BicyclePlugin::on_remove_bicyle)
        .add_observer(BicyclePlugin::init_bicycle)
        // .add_observer(BicyclePlugin::spawn_crank)
        .add_event::<SpawnBicycleEvent>();
        // .add_event::<SpawnCrankEvent>();
    }
}
