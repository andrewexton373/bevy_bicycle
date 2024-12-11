use bevy::app::{Plugin, Startup, Update};

use super::events::SpawnGroupsetEvent;

pub struct GroupsetPlugin;
impl Plugin for GroupsetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (GroupsetPlugin::turn_crank))
            .add_observer(GroupsetPlugin::init_groupset)
            .add_observer(GroupsetPlugin::handle_spawn_component)
            .add_event::<SpawnGroupsetEvent>();
    }
}
