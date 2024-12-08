use avian2d::prelude::*;
use bevy::{
    ecs::system::{ExclusiveSystemParamFunction, RunSystemOnce, SystemState}, input::mouse::{MouseScrollUnit, MouseWheel}, math::{dvec2, DVec2}, prelude::*, sprite::MaterialMesh2dBundle, state::commands, utils::hashbrown::HashMap
};

use crate::CustomMaterial;

use super::{components::{Bicycle, BicycleWheel, Frame}, plugin::BicyclePlugin};

pub struct AttachmentPoints {
    frame_id: Entity,
    bottom_bracket: DVec2,
    front_hub: DVec2,
    rear_hub: DVec2
}

#[derive(Event)]
pub struct SpawnBicycleEvent;

#[derive(Event)]
pub struct SpawnWheelEvent {
    frame_id: Entity,
    wheel: BicycleWheel
}

#[derive(Event)]
pub struct SpawnFrameEvent {
    bicycle_id: Entity,
}

#[derive(Event)]
pub struct SpawnCrankEvent;

#[derive(Event)]
pub struct SpawnAttachmentPointEvent;

#[derive(Component, PartialEq, Eq, Hash)]
pub enum AttachmentPoint {
    FrontWheelFork,
    RearWheelFork,
    BottomBracket
}

impl BicyclePlugin {

    pub fn initialize(mut evt_writer: EventWriter<SpawnBicycleEvent>) {
        evt_writer.send(SpawnBicycleEvent);
    }

    pub fn spawn_wheel(
        mut commands: Commands,
        mut evt_reader: EventReader<SpawnWheelEvent>,
        attachment_points: Query<(Entity, &AttachmentPoint)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut custom_materials: ResMut<Assets<CustomMaterial>>,
        asset_server: Res<AssetServer>,
    ) {

        for evt in evt_reader.read() {

            let (attachment_point_ent, _) =  match evt.wheel {
                BicycleWheel::Front => attachment_points.iter().find(|(_, attachment_point)| *attachment_point == &AttachmentPoint::FrontWheelFork).unwrap(),
                BicycleWheel::Back => attachment_points.iter().find(|(_, attachment_point)| *attachment_point == &AttachmentPoint::RearWheelFork).unwrap(),
            };

            let wheel_ent = commands.entity(attachment_point_ent).with_child((
                evt.wheel,
                RigidBody::Dynamic,
                Collider::circle(BicycleWheel::size() as f64),
                CollisionMargin(1.0),
                Mass::new(1.0),
                Friction::new(0.95),
                Restitution::new(0.0),
                SweptCcd::default(),
                Mesh2d(meshes.add(Circle::new(BicycleWheel::size())).into()),
                MeshMaterial2d(custom_materials.add(CustomMaterial {
                    color: LinearRgba::WHITE,
                    color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                    alpha_mode: AlphaMode::Blend,
                })),
                Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0),
                    ..default()
                }
            )).id();

            commands.entity(attachment_point_ent).with_child(
                RevoluteJoint::new(attachment_point_ent, wheel_ent)
                    // .with_local_anchor_1(attachment_points.front_hub)
                    .with_compliance(0.0)
                    .with_angular_velocity_damping(0.0)
                    .with_linear_velocity_damping(0.0),
            );

        }
   
    }

    

    pub fn spawn_frame(
        mut commands: Commands,
        mut evt_reader: EventReader<SpawnFrameEvent>,
        mut wheel_evt_writer: EventWriter<SpawnWheelEvent>,
        mut crank_evt_writer: EventWriter<SpawnCrankEvent>

    ) {

        for _evt in evt_reader.read() {

            let rear_hub = Vec2::new(-40.0, 0.0);
            let front_hub = Vec2::new(35.0, 0.0);
            let bottom_bracket = Vec2::new(0.0, 0.0);
            let seat_clamp = Vec2::new(-10.0, 20.0);
            let stem_clamp = Vec2::new(30.0, 20.0);

            let frame_points_all: Vec<Vec2> =
                vec![rear_hub, bottom_bracket, seat_clamp, stem_clamp, front_hub];
            let frame_points_all_dvec2 = frame_points_all.iter().map(|v| v.as_dvec2()).collect();
            
            let frame_points_all_indicies: Vec<[u32; 2]> =
                vec![[0, 1], [1, 2], [2, 0], [2, 3], [1, 3], [3, 4]];

            let frame_collider =
                Collider::convex_decomposition(frame_points_all_dvec2, frame_points_all_indicies);

            let frame_id = commands
                .spawn((
                    Frame,
                    RigidBody::Dynamic,
                    frame_collider,
                    Sensor,
                    MassPropertiesBundle {
                        mass: Mass::new(10.0),
                        ..default()
                    },
                )).with_children(|frame| {
                    frame.spawn((AttachmentPoint::BottomBracket, Transform::from_translation(bottom_bracket.extend(0.0))));
                    frame.spawn((AttachmentPoint::FrontWheelFork, Transform::from_translation(front_hub.extend(0.0))));
                    frame.spawn((AttachmentPoint::RearWheelFork, Transform::from_translation(rear_hub.extend(0.0))));
                })
                .id();

            wheel_evt_writer.send(
                SpawnWheelEvent {
                    frame_id,
                    wheel: BicycleWheel::Front
                }
            );

            wheel_evt_writer.send(
                SpawnWheelEvent {
                    frame_id,
                    wheel: BicycleWheel::Back
                }
            );

            crank_evt_writer.send(
                SpawnCrankEvent
            );

        }        

    }
    
    pub fn spawn_crank(
        mut commands: Commands,
        mut evt_reader: EventReader<SpawnCrankEvent>,
        attachment_points: Query<(Entity, &AttachmentPoint)>,
    ) {

        for _evt in evt_reader.read() {

            let (attachment_point_ent, _) =  attachment_points.iter().find(|(_, attachment_point)| *attachment_point == &AttachmentPoint::BottomBracket).unwrap();

            let crank_collider = Collider::polyline(
                vec![
                    8.0 * DVec2::Y,
                    8.0 * DVec2::NEG_Y,
                ],
                vec![[0, 1]].into(),
            );
    
            let crank_ent = commands.entity(attachment_point_ent).with_child((
                RigidBody::Dynamic,
                crank_collider,
                Sensor,
                MassPropertiesBundle {
                    mass: Mass::new(10.0),
                    ..default()
                },
            )).id();

            commands.entity(attachment_point_ent).with_child(
            RevoluteJoint::new(attachment_point_ent, crank_ent)
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
            );
        }
        
        
    }
    
    pub fn spawn_bicycle(
        mut commands: Commands,
        mut evt_reader: EventReader<SpawnBicycleEvent>,
        mut frame_evt_writer: EventWriter<SpawnFrameEvent>
    ) {
        for _evt in evt_reader.read() {
            let bicycle_id = commands.spawn((Bicycle, GlobalTransform::default())).id();
            frame_evt_writer.send(SpawnFrameEvent {bicycle_id});
        }                
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
