use std::f32::consts::PI;

use bevy::{math::vec3, prelude::*};
use avian2d::prelude::*;

pub struct ChainPlugin;

impl Plugin for ChainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ChainPlugin::setup_chain);
    }
}