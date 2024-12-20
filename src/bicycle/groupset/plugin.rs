use avian2d::prelude::PhysicsSet;
use bevy::{
    app::{Plugin, PostUpdate},
    prelude::{in_state, IntoSystemConfigs},
};

use crate::GameState;

use super::{
    events::SpawnGroupsetEvent,
    resources::{CassetteRadius, ChainringRadius},
};

pub struct GroupsetPlugin;
impl Plugin for GroupsetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                GroupsetPlugin::update_chainring_size,
                GroupsetPlugin::update_cassette_size,
                GroupsetPlugin::turn_crank,
                GroupsetPlugin::limit_crank_rpm,
            )
                .chain()
                .after(PhysicsSet::Sync)
                .run_if(in_state(GameState::Ready)),
        )
        .init_resource::<ChainringRadius>()
        .init_resource::<CassetteRadius>()
        .add_observer(GroupsetPlugin::init_groupset)
        .add_observer(GroupsetPlugin::handle_spawn_component)
        .add_event::<SpawnGroupsetEvent>();
    }
}
