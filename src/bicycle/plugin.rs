use bevy::prelude::*;

use super::{
    groupset::GroupsetPlugin,
    systems::{on_remove_bicyle, BicycleSystems},
};

pub struct BicyclePlugin;

impl Plugin for BicyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // SprocketPlugin,
            GroupsetPlugin,
        ))
        .add_observer(on_remove_bicyle)
        .init_resource::<BicycleSystems>();
    }
}
