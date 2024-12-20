use avian2d::prelude::*;
use bevy::{
    math::{DVec2, VectorSpace},
    prelude::*,
};

use crate::{
    camera::components::FollowCamera,
    world::{plugin::WorldTerrainPlugin, resources::TerrainSeed},
};

use super::{
    chain::components::Chain,
    components::{Bicycle, BicycleFrame},
    events::SpawnBicycleEvent,
    groupset::{components::Cog, events::SpawnGroupsetEvent},
    plugin::BicyclePlugin,
    wheel::{components::BicycleWheel, events::SpawnWheelEvent},
};

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    World,
    Frame,
    Wheels,
    AttachmentPoints,
    Groupset,
    Chain,
}

impl BicyclePlugin {
    pub fn spawn_bicycle_on_startup(mut commands: Commands) {
        commands.trigger(SpawnBicycleEvent);
    }

    pub fn init_bicycle(
        _trigger: Trigger<SpawnBicycleEvent>,
        mut commands: Commands,
        bicycle: Query<Entity, With<Bicycle>>,
    ) {
        // Despawn Bicycle If It Already Exists to prepare to reinitialize.
        if let Ok(bicycle_ent) = bicycle.get_single() {
            commands.entity(bicycle_ent).despawn_recursive();
        }

        commands.spawn((
            Bicycle,
            Name::new("Bicycle"),
            Transform::default(),
            InheritedVisibility::default(),
        ));
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

    pub fn spawn_frame(
        trigger: Trigger<OnAdd, Bicycle>,
        mut commands: Commands,
        terrain_seed: Res<TerrainSeed>,
        camera_t: Query<&Transform, With<FollowCamera>>,
    ) {
        let bicycle_ent = trigger.entity();

        let bicycle_frame = BicycleFrame::new();
        let frame_collider = bicycle_frame.collider();

        let mut camera_pos = DVec2::ZERO;
        if let Ok(camera_t) = camera_t.get_single() {
            camera_pos = camera_t.translation.truncate().as_dvec2();
        }

        let spawn_height: f32 =
            30.0 + WorldTerrainPlugin::terrain_height_sample(camera_pos.x, terrain_seed.0) as f32;

        info!("SPAWN HEIGHT: {:?}", spawn_height);

        let frame_id = commands
            .spawn((
                BicycleFrame::new(),
                Name::new("Frame"),
                Transform::from_xyz(camera_pos.x as f32, spawn_height, 0.0),
                RigidBody::Dynamic,
                Rotation::default(),
                Visibility::Inherited,
                frame_collider,
                CollisionLayers::new(GameLayer::Frame, GameLayer::World),
                MassPropertiesBundle {
                    mass: Mass::new(1.0),
                    ..default()
                },
            ))
            .id();

        commands.trigger(SpawnWheelEvent {
            wheel: BicycleWheel::Front,
        });

        commands.trigger(SpawnWheelEvent {
            wheel: BicycleWheel::Back,
        });

        commands.trigger(SpawnGroupsetEvent);

        // commands.trigger(ResetChainEvent);

        // commands.trigger(SpawnCrankEvent);
    }

    // pub fn spawn_crank(
    //     _trigger: Trigger<SpawnCrankEvent>,
    //     mut commands: Commands,
    //     attachment_points: Query<(Entity, &AttachmentPoint)>,
    // ) {

    //     let (attachment_point_ent, attachment_point) =  attachment_points.iter().find(|(_, attachment_point)| *attachment_point == &AttachmentPoint::BottomBracket).unwrap();

    //     let crank_collider = Collider::polyline(
    //         vec![
    //             8.0 * DVec2::Y,
    //             8.0 * DVec2::NEG_Y,
    //         ],
    //         vec![[0, 1]].into(),
    //     );

    //     let crank_ent = commands.spawn((
    //         Name::new("Crank"),
    //         RigidBody::Dynamic,
    //         crank_collider,
    //         Sensor,
    //         Transform::default(),
    //         MassPropertiesBundle {
    //             mass: Mass::new(10.0),
    //             ..default()
    //         },
    //     )).id();

    //     let joint = commands.spawn((
    //         Name::new(attachment_point.name() + "Revolute Joint"),
    //         // RigidBody::Dynamic,
    //         RevoluteJoint::new(attachment_point_ent, crank_ent)
    //             .with_compliance(0.0)
    //             // .with_angular_velocity_damping(0.0)
    //             // .with_linear_velocity_damping(0.0),
    //     )).id();

    //     commands.entity(attachment_point_ent).add_child(joint);

    // }
}
