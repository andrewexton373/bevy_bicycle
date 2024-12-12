use bevy::prelude::*;

use super::events::ResetChainEvent;

pub struct ChainPlugin;

impl Plugin for ChainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ChainPlugin::reset_chain_on_press_r)
            .add_observer(ChainPlugin::reset_chain)
            .add_event::<ResetChainEvent>();
    }
}
