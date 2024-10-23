use bevy::prelude::*;

pub struct ChainPlugin;

impl Plugin for ChainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ChainPlugin::setup_chain);
    }
}
