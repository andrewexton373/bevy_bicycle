use avian2d::prelude::*;
use bevy::{
    ecs::system::{RunSystemOnce, SystemId, SystemState},
    math::DVec2,
    prelude::*,
    utils::HashMap,
};

use crate::{
    bicycle::{groupset::spawn_groupset, wheel::spawn_wheel},
    camera::components::FollowCamera,
    world::{plugin::WorldTerrainPlugin, resources::TerrainSeed},
};

use super::{
    chain::{spawn_chain, Chain},
    components::{Bicycle, BicycleFrame},
    frame::spawn_frame,
    groupset::Cog,
    plugin::BicyclePlugin,
    wheel::BicycleWheel,
};

use std::collections::BTreeMap; // itertools = "0.8"

use avian2d::prelude::{Collider, Position, Rotation};
use bevy::{math::DVec2, prelude::Component};

#[derive(Component)]
pub struct Bicycle;

#[derive(Resource)]
pub struct BicycleSystems(pub HashMap<String, SystemId>);

impl FromWorld for BicycleSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = BicycleSystems(HashMap::new());

        systems
            .0
            .insert("spawn_bicycle".into(), world.register_system(spawn_bicycle));

        systems
            .0
            .insert("spawn_chain".into(), world.register_system(spawn_chain));

        systems
    }
}

fn spawn_bicycle(world: &mut World, mut bicycle: QueryState<Entity, With<Bicycle>>) {
    // Despawn Bicycle If It Already Exists to prepare to reinitialize.
    if let Ok(bicycle_ent) = bicycle.get_single_mut(world) {
        world.entity_mut(bicycle_ent).despawn_recursive();
    }

    world.spawn((
        Bicycle,
        Name::new("Bicycle"),
        Transform::default(),
        InheritedVisibility::default(),
    ));

    world
        .run_system_once(spawn_frame)
        .expect("Error Spawning Frame");
}

pub fn on_remove_bicyle(
    _trigger: Trigger<OnRemove, Bicycle>,
    mut commands: Commands,
    frame: Query<Entity, With<BicycleFrame>>,
    wheels: Query<Entity, With<BicycleWheel>>,
    cogs: Query<Entity, With<Cog>>,
    chain: Query<Entity, With<Chain>>,
    rev_joints: Query<Entity, With<RevoluteJoint>>,
    fixed_joints: Query<Entity, With<FixedJoint>>,
) {
    commands.entity(frame.single()).despawn_recursive();

    for ent in wheels.iter() {
        commands.entity(ent).despawn_recursive();
    }

    for ent in cogs.iter() {
        commands.entity(ent).despawn_recursive();
    }

    if chain.iter().count() > 0 {
        commands.entity(chain.single()).try_despawn_recursive();
    }

    for ent in rev_joints.iter() {
        commands.entity(ent).despawn_recursive();
    }

    for ent in fixed_joints.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
