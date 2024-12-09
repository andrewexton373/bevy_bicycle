use avian2d::prelude::*;
use bevy::{color::palettes::css::{GREEN, RED, WHEAT}, ecs::entity, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, state::commands};

use crate::{bicycle::{groupset::events::SpawnAttachedEvent, systems::{AttachmentPoint, GameLayer}}, CustomMaterial};

use super::{components::{Axle, Disc, Groupset, Point, Radius}, events::SpawnGroupsetEvent, plugin::GroupsetPlugin};

impl GroupsetPlugin {

    pub fn init_groupset(
        trigger: Trigger<SpawnGroupsetEvent>,
        attachment_points: Query<(Entity, &AttachmentPoint, &Transform)>,
        mut commands: Commands
    ) {

        let groupset = commands.spawn((
            Groupset,
            Name::new("Groupset")
        )).id();


        for (ent, attachment_point, t) in attachment_points.iter() {
            match attachment_point {
                // AttachmentPoint::FrontWheelFork => todo!(),
                // AttachmentPoint::RearWheelFork => todo!(),
                AttachmentPoint::BottomBracket | AttachmentPoint::RearWheelFork => {
                    commands.entity(ent).trigger(SpawnAttachedEvent);
                },
                _ => {

                }
            }
        }

    }

    pub fn handle_spawn_component(
        trigger: Trigger<SpawnAttachedEvent>,
        mut commands: Commands,
        attachment_points: Query<(Entity, &AttachmentPoint, &Transform)>,
        meshes: ResMut<Assets<Mesh>>,
        color_materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let (ent, attachment_point, t) = attachment_points.get(trigger.entity()).unwrap();

        match attachment_point {
            AttachmentPoint::BottomBracket => {
                let front_axel = commands.spawn(GroupsetPlugin::front_axle(meshes, color_materials, t)).id();
                commands.entity(front_axel).with_child(
                    (
                        Name::new("Bottom Bracket / Chainring Revolute Joint"),
                        RevoluteJoint::new(ent, front_axel)
                            // .with_local_anchor_1(t.translation.xy().as_dvec2())
                            .with_angular_velocity_damping(0.0)
                            .with_linear_velocity_damping(0.0)
                    )
                );

                
                // commands.entity(ent).add_child(id);
                // commands.entity(ent).with_child(
                //     (
                //         Name::new("Bottom Bracket / Chainring Revolute Joint"),
                //         RevoluteJoint::new(ent, id)
                //             .with_local_anchor_1(t.translation.xy().as_dvec2())
                //             .with_angular_velocity_damping(0.0)
                //             .with_linear_velocity_damping(0.0)
                //     )
                // );
            },
            AttachmentPoint::RearWheelFork => {
                let rear_wheel_fork = commands.spawn(GroupsetPlugin::back_axle(meshes, color_materials, t)).id();

                commands.entity(rear_wheel_fork).with_child((
                    Name::new("Rear Wheel Fork / Cassette Revolute Joint"),
                    RevoluteJoint::new(ent, rear_wheel_fork)
                        // .with_local_anchor_1(t.translation.xy().as_dvec2())
                        .with_angular_velocity_damping(0.0)
                        .with_linear_velocity_damping(0.0)
                ));
            }
            _ => {}
        }
    }

    pub fn front_axle(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
        t: &Transform
    ) -> impl Bundle {
        let wheel_size = 5.0;

        (
            Axle::FRONT,
            Name::new("Front Axel (Chainring)"),
            Disc {
                center: Point {x: 0.0, y: 0.0},
                radius: wheel_size
            },
            Radius(wheel_size as f32),
            RigidBody::Dynamic,
            Collider::circle(wheel_size as f64),
            CollisionMargin(1.0),
            Mass::new(1.0),
            Friction::new(0.99),
            Restitution::new(0.0),
            SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
            Mesh2d(meshes.add(Circle::new(wheel_size as f32)).into()),
            // MeshMaterial2d(custom_materials.add(CustomMaterial {
            //     color: LinearRgba::WHITE,
            //     color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
            //     alpha_mode: AlphaMode::Blend,
            // })),
            MeshMaterial2d(color_materials.add(ColorMaterial::from_color(GREEN))),
            CollisionLayers::new(GameLayer::Groupset, GameLayer::Groupset),

            t.clone()

        )
    }

    pub fn spin_front_axle(
        mut axles: Query<(&Axle, &mut AngularVelocity), With<Axle>>,
        mut mouse_wheel_evt: EventReader<MouseWheel>,

    ) {
        for &evt in mouse_wheel_evt.read() {
            match &evt.unit {
                MouseScrollUnit::Line => {
                    for (axle, mut ang_vel) in axles.iter_mut() {
                        if let Axle::FRONT = axle {
                            ang_vel.0 += -1.0_f64 * (evt.y as f64);
                            // ang_vel.0 += -10.0 as f64 * evt.y as f64;
                            println!("ang_vel {}", ang_vel.0);
                        }
                    }
                }
                MouseScrollUnit::Pixel => {}
            }
        }
    }

    pub fn back_axle(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
        t: &Transform
    ) -> impl Bundle {

        let wheel_size = 4.0;

        (
            Axle::REAR,
            Name::new("Rear Axel (Cassette)"),
            Disc {
                center: Point {x: 0.0, y: 0.0},
                radius: wheel_size
            },
            Radius(wheel_size as f32),
            RigidBody::Dynamic,
            Collider::circle(wheel_size as f64),
            CollisionMargin(1.0),
            Mass::new(1.0),
            Friction::new(0.99),
            Restitution::new(0.0),
            SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
            Mesh2d(meshes.add(Circle::new(wheel_size as f32)).into()),
            // MeshMaterial2d(custom_materials.add(CustomMaterial {
            //     color: LinearRgba::WHITE,
            //     color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
            //     alpha_mode: AlphaMode::Blend,
            // })),
            MeshMaterial2d(color_materials.add(ColorMaterial::from_color(RED))),
            CollisionLayers::new(GameLayer::Groupset, GameLayer::Groupset),

            t.clone()
        )
    }


}