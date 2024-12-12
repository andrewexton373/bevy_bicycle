
use avian2d::prelude::*;
use bevy::prelude::*;

use super::{
    components::{Bicycle, BicycleFrame},
    groupset::events::SpawnGroupsetEvent,
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
    Chain
}

impl BicyclePlugin {
    pub fn init_bicycle(mut commands: Commands) {
        commands.spawn((
            Bicycle,
            Name::new("Bicycle"),
            Transform::default(),
            InheritedVisibility::default(),
        ));
    }

    pub fn spawn_frame(trigger: Trigger<OnAdd, Bicycle>, mut commands: Commands) {
        let bicycle_ent = trigger.entity();

        let bicycle_frame = BicycleFrame::new();
        let frame_collider = bicycle_frame.collider();

        let frame_id = commands
            .spawn((
                BicycleFrame::new(),
                Name::new("Frame"),
                Transform::default(),
                RigidBody::Dynamic,
                Visibility::Inherited,
                frame_collider,
                CollisionLayers::new(GameLayer::Frame, GameLayer::World),
                // Sensor,
                MassPropertiesBundle {
                    mass: Mass::new(10.0),
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
