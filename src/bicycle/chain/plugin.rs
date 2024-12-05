use bevy::prelude::*;

pub struct ChainPlugin;

impl Plugin for ChainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ChainPlugin::reset_chain);
    }
}
