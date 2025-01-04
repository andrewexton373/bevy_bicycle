use bevy::prelude::*;

#[derive(Component)]
pub struct Terrain;

#[derive(Component, PartialEq)]
pub struct TerrainChunk(pub i128);
