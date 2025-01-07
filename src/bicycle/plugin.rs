use bevy::prelude::*;

use super::{groupset::GroupsetPlugin, systems::BicycleSystems};

pub struct BicyclePlugin;

impl Plugin for BicyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // SprocketPlugin,
            GroupsetPlugin,
        ))
        .add_observer(BicyclePlugin::on_remove_bicyle)
        .init_resource::<BicycleSystems>();
    }
}
