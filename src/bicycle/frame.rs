use avian2d::prelude::*;
use bevy::ecs::system::{RunSystemOnce, SystemState};
use bevy::math::DVec2;
use bevy::prelude::*;

use crate::bicycle::components::BicycleFrame;
use crate::bicycle::groupset::spawn_groupset;
use crate::bicycle::systems::GameLayer;
use crate::bicycle::wheel::{spawn_wheel, BicycleWheel};
use crate::camera::components::FollowCamera;
use crate::world::plugin::WorldTerrainPlugin;
use crate::world::resources::TerrainSeed;

pub fn spawn_frame(world: &mut World) {
    let mut system_state: SystemState<(Res<TerrainSeed>, Query<&Transform, With<FollowCamera>>)> =
        SystemState::new(world);
    let (terrain_seed, camera_t) = system_state.get_mut(world);

    let bicycle_frame = BicycleFrame::new();
    let frame_collider = bicycle_frame.collider();

    let mut camera_pos = DVec2::ZERO;
    if let Ok(camera_t) = camera_t.get_single() {
        camera_pos = camera_t.translation.truncate().as_dvec2();
    }

    let spawn_height: f32 = 50.0
        + WorldTerrainPlugin::CHUNK_WIDTH
            * WorldTerrainPlugin::terrain_height_sample(camera_pos.x, terrain_seed.0) as f32;

    info!("SPAWN HEIGHT: {:?}", spawn_height);

    let _frame_id = world
        .spawn((
            BicycleFrame::new(),
            Name::new("Frame"),
            Transform::from_xyz(camera_pos.x as f32, spawn_height, 0.0),
            RigidBody::Dynamic,
            Mass(10.0),
            AngularInertia(0.1),
            CenterOfMass(bevy::prelude::Vec2::ZERO),
            Visibility::Inherited,
            frame_collider,
            CollisionMargin(0.5),
            CollisionLayers::new([GameLayer::Frame], [GameLayer::World]),
        ))
        .id();

    world
        .run_system_once_with(BicycleWheel::Front, spawn_wheel)
        .expect("Error Spawning Front Wheel");
    world
        .run_system_once_with(BicycleWheel::Back, spawn_wheel)
        .expect("Error Spawning Rear Wheel");
    world
        .run_system_once(spawn_groupset)
        .expect("Error Spawning Groupset");

    // TODO: causes lag due to physics interactions
    // world.run_system_once(ChainPlugin::spawn_chain);
}
