use bevy::prelude::Resource;

#[derive(Resource, PartialEq)]
pub struct ChainringRadius(pub f32);

impl Default for ChainringRadius {
    fn default() -> Self {
        ChainringRadius(5.0)
    }
}

#[derive(Resource, PartialEq)]
pub struct CassetteRadius(pub f32);

impl Default for CassetteRadius {
    fn default() -> Self {
        CassetteRadius(5.0)
    }
}
