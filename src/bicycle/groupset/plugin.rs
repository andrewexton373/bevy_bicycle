use bevy::app::{Plugin, Startup, Update};

pub struct GroupsetPlugin;
impl Plugin for GroupsetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, (
                GroupsetPlugin::spawn_front_axle,
                GroupsetPlugin::spawn_back_axle
            ))
            .add_systems(Update, (
                GroupsetPlugin::spin_front_axle
            ));
    }
}