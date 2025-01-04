use bevy::prelude::*;

use super::events::ResetChainEvent;

pub struct ChainPlugin;

impl Plugin for ChainPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetChainEvent>()
            .add_observer(ChainPlugin::reset_chain);
    }
}
