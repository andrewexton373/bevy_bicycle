use std::iter::Map;

use avian2d::prelude::*;
use bevy::{
    color::palettes::css::{BLACK, BLUE, GREEN, PINK}, ecs::system::{ExclusiveSystemParamFunction, RunSystemOnce, SystemState}, input::mouse::{MouseScrollUnit, MouseWheel}, math::{dvec2, DVec2}, prelude::*, sprite::MaterialMesh2dBundle, state::commands, utils::hashbrown::HashMap
};

use crate::CustomMaterial;

use super::{components::{Bicycle, BicycleWheel, Frame}, groupset::events::SpawnGroupsetEvent, plugin::BicyclePlugin};

#[derive(Event)]
pub struct SpawnBicycleEvent;

#[derive(Event)]
pub struct SpawnWheelEvent {
    wheel: BicycleWheel
}

#[derive(Event)]
pub struct SpawnFrameEvent;

#[derive(Event)]
pub struct SpawnCrankEvent;

#[derive(Event)]
pub struct SpawnAttachmentPointEvent;

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
    BottomBracket,
    WheelCassette
}

impl AttachmentPoint {
    pub fn name(&self) -> String {
        match self {
            AttachmentPoint::FrontWheelFork => "Front Wheel Fork".to_string(),
            AttachmentPoint::RearWheelFork => "Rear Wheel Fork".to_string(),
            AttachmentPoint::BottomBracket => "Bottom Bracket".to_string(),
            AttachmentPoint::WheelCassette => "Wheel Cassette".to_string()
        }
    }
}

#[derive(Component)]
pub struct BicycleFrame {
    pub rear_hub: Vec2,
    pub front_hub: Vec2,
    pub bottom_bracket: Vec2,
    pub seat_clamp: Vec2,
    pub stem_clamp: Vec2,
}

impl BicycleFrame {
    pub fn new() -> Self {
        BicycleFrame {
            rear_hub: Vec2::new(-40.0, 0.0),
            front_hub: Vec2::new(35.0, 0.0),
            bottom_bracket: Vec2::new(0.0, 0.0),
            seat_clamp: Vec2::new(-10.0, 20.0),
            stem_clamp: Vec2::new(30.0, 20.0),
        }
    }

    pub fn collider(&self) -> Collider {
        let frame_points_all: Vec<Vec2> =
        vec![self.rear_hub, self.bottom_bracket, self.seat_clamp, self.stem_clamp, self.front_hub];
        let frame_points_all_dvec2 = frame_points_all.iter().map(|v| v.as_dvec2()).collect();
        
        let frame_points_all_indicies: Vec<[u32; 2]> =
            vec![[0, 1], [1, 2], [2, 0], [2, 3], [1, 3], [3, 4]];

        let frame_collider =
            Collider::convex_decomposition(frame_points_all_dvec2, frame_points_all_indicies);

        frame_collider
    }

    pub fn attachment_points(&self) -> HashMap<AttachmentPoint, Vec2> {
        let mut attachment_points = HashMap::new();
        attachment_points.insert(AttachmentPoint::BottomBracket, self.bottom_bracket);
        attachment_points.insert(AttachmentPoint::FrontWheelFork, self.front_hub);
        attachment_points.insert(AttachmentPoint::RearWheelFork, self.rear_hub);
        attachment_points.insert(AttachmentPoint::WheelCassette, self.rear_hub);

        
        attachment_points.clone()
    }
}

impl BicyclePlugin {

    pub fn init_bicycle(
        mut commands: Commands,
    ) {
        commands.spawn((
            Bicycle,
            RigidBody::Dynamic,
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
                DebugRender::default().with_collider_color(BLUE.into()),
                Visibility::Inherited,
                Transform::from_translation(pos.extend(0.0)),
            )).id();

            // commands.entity(frame_id).add_child(attachment_ent);

            commands.spawn(FixedJoint::new(frame_id, attachment_ent).with_local_anchor_1(pos.as_dvec2()));

        }

        // commands.entity(bicycle_ent).add_child(frame_id);

        commands.trigger(SpawnWheelEvent {
            wheel: BicycleWheel::Front
        });

        commands.trigger(SpawnWheelEvent {
            wheel: BicycleWheel::Back
        });

        // commands.trigger(SpawnCrankEvent);

        commands.trigger(SpawnGroupsetEvent);

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

    pub fn spawn_wheel(
        trigger: Trigger<SpawnWheelEvent>,
        mut commands: Commands,
        attachment_points: Query<(Entity, &AttachmentPoint, &Transform)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut custom_materials: ResMut<Assets<CustomMaterial>>,
        asset_server: Res<AssetServer>,
    ) {

        let evt = trigger.event();

        let (attachment_point_ent, _, t) =  match evt.wheel {
            BicycleWheel::Front => attachment_points.iter().find(|(_, attachment_point, _)| *attachment_point == &AttachmentPoint::FrontWheelFork).unwrap(),
            BicycleWheel::Back => attachment_points.iter().find(|(_, attachment_point, _)| *attachment_point == &AttachmentPoint::RearWheelFork).unwrap(),
        };

        let wheel = commands.spawn((
            evt.wheel,
            Name::new("Wheel"),
            RigidBody::Dynamic,
            Collider::circle(BicycleWheel::size() as f64),
            DebugRender::default().with_collider_color(BLACK.into()),
            CollisionMargin(1.0),
            Mass::new(1.0),
            Friction::new(0.95),
            Restitution::new(0.0),
            SweptCcd::default(),
            Mesh2d(meshes.add(Circle::new(BicycleWheel::size())).into()),
            // CollisionLayers::new(GameLayer::Wheels.to_bits(), GameLayer::World)
            MeshMaterial2d(custom_materials.add(CustomMaterial {
                color: LinearRgba::WHITE,
                color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                alpha_mode: AlphaMode::Blend,
            })),
            t.clone()
        )).id();

        let joint = commands.spawn((
            Name::new("Wheel Joint"),
            RevoluteJoint::new(attachment_point_ent, wheel)
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
        )).id();
   
    }
    
    pub fn spawn_crank(
        _trigger: Trigger<SpawnCrankEvent>,
        mut commands: Commands,
        attachment_points: Query<(Entity, &AttachmentPoint)>,
    ) {

        let (attachment_point_ent, attachment_point) =  attachment_points.iter().find(|(_, attachment_point)| *attachment_point == &AttachmentPoint::BottomBracket).unwrap();

        let crank_collider = Collider::polyline(
            vec![
                8.0 * DVec2::Y,
                8.0 * DVec2::NEG_Y,
            ],
            vec![[0, 1]].into(),
        );

        let crank_ent = commands.spawn((
            Name::new("Crank"),
            RigidBody::Dynamic,
            crank_collider,
            Sensor,
            Transform::default(),
            MassPropertiesBundle {
                mass: Mass::new(10.0),
                ..default()
            },
        )).id();

        let joint = commands.spawn((
            Name::new(attachment_point.name() + "Revolute Joint"),
            // RigidBody::Dynamic,
            RevoluteJoint::new(attachment_point_ent, crank_ent)
                .with_compliance(0.0)
                // .with_angular_velocity_damping(0.0)
                // .with_linear_velocity_damping(0.0),
        )).id();

        commands.entity(attachment_point_ent).add_child(joint);
        
    }
    
    

    pub fn spin_wheel(
        mut wheel_query: Query<(&BicycleWheel, &mut ExternalTorque), With<BicycleWheel>>,
        mut mouse_wheel_evt: EventReader<MouseWheel>,
    ) {
        for &evt in mouse_wheel_evt.read() {
            match &evt.unit {
                MouseScrollUnit::Line => {
                    for (wheel, mut torque) in wheel_query.iter_mut() {
                        if let BicycleWheel::Back = wheel {
                            *torque = ExternalTorque::new(-2000000.0_f64 * evt.y as f64)
                                .with_persistence(true);
                            // ang_vel.0 += -10.0 as f64 * evt.y as f64;
                            println!("torque {}", torque.torque());
                        }
                    }
                }
                MouseScrollUnit::Pixel => {}
            }
        }
    }
}
