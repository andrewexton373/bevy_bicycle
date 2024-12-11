use bevy::app::Plugin;

use crate::bicycle::plugin::BicyclePlugin;

use super::events::SpawnWheelEvent;

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnWheelEvent>()
            .add_observer(WheelPlugin::spawn_wheel);
    }
}

impl WheelPlugin {}
