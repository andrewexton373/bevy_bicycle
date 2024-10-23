use bevy::prelude::*;
use avian2d::prelude::*;

pub struct SprocketPlugin;

impl Plugin for SprocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, SprocketPlugin::setup_sproket);
    }
}