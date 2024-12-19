use bevy::prelude::*;

use super::resources::{MaxTerrainChunkCount, TerrainSeed};

pub struct WorldTerrainPlugin;

impl Plugin for WorldTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainSeed>()
            .add_systems(
                Update,
                (
                    WorldTerrainPlugin::generate_surrounding_terrain_chunks,
                    WorldTerrainPlugin::remove_chunks_outside_viewport,
                ),
            )
            .init_resource::<MaxTerrainChunkCount>();
    }
}
