use avian2d::prelude::PhysicsSet;
use bevy::{app::{Plugin, PostUpdate, Update}, prelude::IntoSystemConfigs, text::cosmic_text::ttf_parser::gsub::Sequence};

use super::{events::SpawnGroupsetEvent, resources::{CassetteRadius, ChainringRadius}};

pub struct GroupsetPlugin;
impl Plugin for GroupsetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PostUpdate,
        (
                    GroupsetPlugin::turn_crank,
                    GroupsetPlugin::limit_crank_rpm
                )
                .chain().after(PhysicsSet::Sync)
            )
            .init_resource::<ChainringRadius>()
            .init_resource::<CassetteRadius>()
            .add_observer(GroupsetPlugin::init_groupset)
            .add_observer(GroupsetPlugin::handle_spawn_component)
            .add_event::<SpawnGroupsetEvent>();
    }
}
