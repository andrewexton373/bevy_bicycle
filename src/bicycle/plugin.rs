use bevy::prelude::*;

use super::{
    groupset::plugin::GroupsetPlugin, systems::BicycleSystems, wheel::plugin::WheelPlugin,
};

pub struct BicyclePlugin;

impl Plugin for BicyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // SprocketPlugin,
            GroupsetPlugin,
            WheelPlugin,
        ))
        .add_observer(BicyclePlugin::on_remove_bicyle)
        .init_resource::<BicycleSystems>();
    }
}
