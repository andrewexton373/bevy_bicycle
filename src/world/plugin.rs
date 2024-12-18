use bevy::prelude::*;

use super::resources::TerrainSeed;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {


        app
            .init_resource::<TerrainSeed>()
            .add_systems(
            Update,
            (
                WorldPlugin::generate_surrounding_terrain_chunks,
                WorldPlugin::remove_chunks_outside_viewport,
            ),
        );
    }
}
