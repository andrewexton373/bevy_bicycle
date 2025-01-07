use avian2d::prelude::*;
use bevy::{
    ecs::system::{RunSystemOnce, SystemId, SystemState},
    math::DVec2,
    prelude::*,
    utils::HashMap,
};

use crate::{
    bicycle::{groupset::plugin::GroupsetPlugin, wheel::spawn_wheel},
    camera::components::FollowCamera,
    world::{plugin::WorldTerrainPlugin, resources::TerrainSeed},
};

use super::{
    chain::{spawn_chain, Chain},
    components::{Bicycle, BicycleFrame},
    groupset::components::Cog,
    plugin::BicyclePlugin,
    wheel::BicycleWheel,
};

pub(crate) fn initialize(world: &mut World) {
    world
        .run_system(world.resource::<BicycleSystems>().0["spawn_bicycle"])
        .expect("Error Spawning Bicycle");
}

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

fn spawn_bicycle(world: &mut World) {
    let mut system_state: SystemState<(Query<Entity, With<Bicycle>>)> = SystemState::new(world);
    let (bicycle) = system_state.get_mut(world);

    // Despawn Bicycle If It Already Exists to prepare to reinitialize.
    if let Ok(bicycle_ent) = bicycle.get_single() {
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
        .run_system_once(GroupsetPlugin::spawn_groupset)
        .expect("Error Spawning Groupset");

    // TODO: causes lag due to physics interactions
    // world.run_system_once(ChainPlugin::spawn_chain);
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    World,
    Frame,
    Wheels,
    AttachmentPoints,
    Groupset,
    Chain,
}

impl BicyclePlugin {
    // pub fn spawn_bicycle_on_startup(mut commands: Commands, bicycle: Query<Entity, With<Bicycle>>) {
    //     if bicycle.is_empty() {
    //         info!("SPAWN BICYCLE");
    //         commands.trigger(SpawnBicycleEvent);
    //     }
    // }

    // pub fn init_bicycle(
    //     _trigger: Trigger<SpawnBicycleEvent>,
    //     mut commands: Commands,
    //     bicycle: Query<Entity, With<Bicycle>>,
    // ) {
    //     // Despawn Bicycle If It Already Exists to prepare to reinitialize.
    //     if let Ok(bicycle_ent) = bicycle.get_single() {
    //         commands.entity(bicycle_ent).despawn_recursive();
    //     }
    //
    //     commands.spawn((
    //         Bicycle,
    //         Name::new("Bicycle"),
    //         Transform::default(),
    //         InheritedVisibility::default(),
    //     ));
    // }

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

    // pub fn spawn_frame(
    //     _trigger: Trigger<OnAdd, Bicycle>,
    //     mut commands: Commands,
    //     terrain_seed: Res<TerrainSeed>,
    //     camera_t: Query<&Transform, With<FollowCamera>>,
    // ) {
    //     let bicycle_frame = BicycleFrame::new();
    //     let frame_collider = bicycle_frame.collider();
    //
    //     let mut camera_pos = DVec2::ZERO;
    //     if let Ok(camera_t) = camera_t.get_single() {
    //         camera_pos = camera_t.translation.truncate().as_dvec2();
    //     }
    //
    //     let spawn_height: f32 = 50.0
    //         + WorldTerrainPlugin::CHUNK_WIDTH
    //             * WorldTerrainPlugin::terrain_height_sample(camera_pos.x, terrain_seed.0) as f32;
    //
    //     info!("SPAWN HEIGHT: {:?}", spawn_height);
    //
    //     let _frame_id = commands
    //         .spawn((
    //             BicycleFrame::new(),
    //             Name::new("Frame"),
    //             Transform::from_xyz(camera_pos.x as f32, spawn_height, 0.0),
    //             RigidBody::Dynamic,
    //             Mass(10.0),
    //             AngularInertia(0.1),
    //             CenterOfMass(bevy::prelude::Vec2::ZERO),
    //             Visibility::Inherited,
    //             frame_collider,
    //             CollisionMargin(0.5),
    //             CollisionLayers::new([GameLayer::Frame], [GameLayer::World]),
    //         ))
    //         .id();
    //
    //     commands.trigger(SpawnWheelEvent {
    //         wheel: BicycleWheel::Front,
    //     });
    //
    //     commands.trigger(SpawnWheelEvent {
    //         wheel: BicycleWheel::Back,
    //     });
    //
    //     commands.trigger(SpawnGroupsetEvent);
    //
    //     // commands.trigger(ResetChainEvent);
    //
    //     // commands.trigger(SpawnCrankEvent);
    // }

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
