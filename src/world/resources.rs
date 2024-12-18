use bevy::prelude::Resource;
use rand::RngCore;

#[derive(Resource)]
pub struct TerrainSeed(pub u32);

impl Default for TerrainSeed {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        TerrainSeed(rng.next_u32())
    }
}