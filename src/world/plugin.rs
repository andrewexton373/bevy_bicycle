use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, WorldPlugin::generate_surrounding_terrain_chunks);
    }
}
