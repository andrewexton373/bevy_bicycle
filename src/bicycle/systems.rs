use std::iter::Map;

use avian2d::prelude::*;
use bevy::{
    color::palettes::css::{BLACK, GREEN}, ecs::system::{ExclusiveSystemParamFunction, RunSystemOnce, SystemState}, input::mouse::{MouseScrollUnit, MouseWheel}, math::{dvec2, DVec2}, prelude::*, sprite::MaterialMesh2dBundle, state::commands, utils::hashbrown::HashMap
};

use crate::CustomMaterial;

use super::{components::{Bicycle, BicycleFrame, Frame}, groupset::events::SpawnGroupsetEvent, plugin::BicyclePlugin, wheel::{components::BicycleWheel, events::SpawnWheelEvent}};



#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    World,
    Frame,
    Wheels,
    AttachmentPoints,
    Groupset
}



#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum AttachmentPoint {
    FrontWheelFork,
    RearWheelFork,
    BottomBracket
}

impl AttachmentPoint {
    pub fn name(&self) -> String {
        match self {
            AttachmentPoint::FrontWheelFork => "Front Wheel Fork".to_string(),
            AttachmentPoint::RearWheelFork => "Rear Wheel Fork".to_string(),
            AttachmentPoint::BottomBracket => "Bottom Bracket".to_string(),
        }
    }
}



impl BicyclePlugin {

    pub fn init_bicycle(
        mut commands: Commands,
    ) {
        commands.spawn((
            Bicycle,
            Name::new("Bicycle"),
            Transform::default(),
            InheritedVisibility::default()
        ));           
    }

    

    pub fn spawn_frame(
        trigger: Trigger<OnAdd, Bicycle>,
        mut commands: Commands,
    ) {

        let bicycle_ent = trigger.entity();

        let bicycle_frame = BicycleFrame::new();
        let frame_collider = bicycle_frame.collider();

        let frame_id = commands.spawn((
            Frame,
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
        )).id();

        for (attachment_point, pos) in bicycle_frame.attachment_points().iter() {

            let attachment_ent = commands.spawn((
                *attachment_point,
                Name::new(attachment_point.name()),
                RigidBody::Dynamic,
                Collider::circle(2.0),
                DebugRender::default().with_collider_color(GREEN.into()),
                Visibility::Inherited,
                Transform::from_translation(pos.extend(0.0)),
            )).id();

            commands.spawn(
                RevoluteJoint::new(frame_id, attachment_ent)
                    .with_local_anchor_1(pos.as_dvec2())
                    .with_angular_velocity_damping(0.0)
                    .with_linear_velocity_damping(0.0)
            );
        }

        commands.entity(bicycle_ent).add_child(frame_id);

        commands.trigger(SpawnGroupsetEvent);


        commands.trigger(SpawnWheelEvent {
            wheel: BicycleWheel::Front
        });

        commands.trigger(SpawnWheelEvent {
            wheel: BicycleWheel::Back
        });

        // commands.trigger(SpawnCrankEvent);


    }

    // pub fn attachment_point(
    //     attachment_point
    // ) -> impl Bundle {
    //     (
    //         AttachmentPoint::BottomBracket,
    //         Name::new("Bottom Bracket"),
    //         RigidBody::Dynamic,
    //         Sensor,
    //         Collider::circle(1.0),
    //         Visibility::Inherited,
    //         Transform::from_translation(bottom_bracket.extend(0.0))
    //     )
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
