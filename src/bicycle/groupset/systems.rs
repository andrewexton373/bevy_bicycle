use avian2d::prelude::*;
use bevy::{color::palettes::css::{GREEN, RED, WHEAT}, ecs::entity, input::{keyboard::KeyboardInput, mouse::{MouseScrollUnit, MouseWheel}}, prelude::*, state::commands};

use crate::{bicycle::{groupset::events::SpawnAttachedEvent, systems::{AttachmentPoint, GameLayer}}, CustomMaterial};

use super::{components::{Axle, Cog, Disc, Groupset, Point, Radius}, events::SpawnGroupsetEvent, plugin::GroupsetPlugin};

impl GroupsetPlugin {

    pub fn init_groupset(
        trigger: Trigger<SpawnGroupsetEvent>,
        attachment_points: Query<(Entity, &AttachmentPoint)>,
        mut commands: Commands
    ) {

        let groupset = commands.spawn((
            Groupset,
            Name::new("Groupset")
        )).id();


        for (ent, attachment_point) in attachment_points.iter() {
            match attachment_point {
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

        println!("{:?}", attachment_point);

        match attachment_point {
            AttachmentPoint::BottomBracket => {
                let front_chainring = commands.spawn(GroupsetPlugin::front_chainring(meshes, color_materials, t)).id();
                // commands.entity(front_chainring).with_child(
                //     (
                //         Name::new("Bottom Bracket / Chainring Revolute Joint"),
                //         RevoluteJoint::new(ent, front_chainring)
                //             .with_angular_velocity_damping(0.0)
                //             .with_linear_velocity_damping(0.0)
                //     )
                // );

                commands.spawn(
                    (
                        Name::new("Bottom Bracket / Chainring Revolute Joint"),
                        RevoluteJoint::new(ent, front_chainring)
                            .with_angular_velocity_damping(0.0)
                            .with_linear_velocity_damping(0.0)
                    )
                );
            },
            AttachmentPoint::RearWheelFork => {
                let rear_cassette = commands.spawn(GroupsetPlugin::rear_cassette(meshes, color_materials, t)).id();

                // commands.entity(rear_cassette).with_child((
                //     Name::new("Rear Wheel Fork / Cassette Revolute Joint"),
                //     RevoluteJoint::new(ent, rear_cassette)
                //         .with_angular_velocity_damping(0.0)
                //         .with_linear_velocity_damping(0.0)
                // ));

                commands.spawn((
                    Name::new("Rear Wheel Fork / Cassette Revolute Joint"),
                    RevoluteJoint::new(ent, rear_cassette)
                        .with_angular_velocity_damping(0.0)
                        .with_linear_velocity_damping(0.0)
                ));
            }
            _ => {println!("HIT!");}
        }
    }

    pub fn front_chainring(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
        t: &Transform
    ) -> impl Bundle {
        let wheel_radius = Radius(5.0);
        (
            // Axle::FRONT,
            Cog::FrontChainring,
            Name::new("Front Chainring"),
            wheel_radius,
            RigidBody::Dynamic,
            Collider::circle(wheel_radius.0 as f64),
            CollisionMargin(1.0),
            Mass::new(1.0),
            Friction::new(0.99),
            Restitution::new(0.0),
            SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
            Mesh2d(meshes.add(Circle::new(wheel_radius.0 as f32)).into()),
            // MeshMaterial2d(custom_materials.add(CustomMaterial {
            //     color: LinearRgba::WHITE,
            //     color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
            //     alpha_mode: AlphaMode::Blend,
            // })),
            MeshMaterial2d(color_materials.add(ColorMaterial::from_color(GREEN))),
            CollisionLayers::new(GameLayer::Groupset, GameLayer::Groupset.to_bits() | GameLayer::World.to_bits()),
            // GlobalTransform::default(),
            t.clone()

        )
    }

    pub fn turn_crank(
        mut cogs: Query<(&Cog, &mut AngularVelocity), With<Cog>>,
        mut mouse_wheel_evt: EventReader<MouseWheel>,
    ) {
        for &evt in mouse_wheel_evt.read() {
            match &evt.unit {
                MouseScrollUnit::Line => {
                    for (cog, mut ang_vel) in cogs.iter_mut() {
                        if let Cog::FrontChainring = cog {
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

    pub fn rear_cassette(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
        t: &Transform
    ) -> impl Bundle {

        let wheel_radius = Radius(10.0);

        (
            // Axle::REAR,
            Cog::RearCassette,
            Name::new("Rear Cassette"),
            wheel_radius,
            RigidBody::Dynamic,
            Collider::circle(wheel_radius.0 as f64),
            CollisionMargin(1.0),
            Mass::new(1.0),
            Friction::new(0.99),
            Restitution::new(0.0),
            SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
            Mesh2d(meshes.add(Circle::new(wheel_radius.0 as f32)).into()),
            // MeshMaterial2d(custom_materials.add(CustomMaterial {
            //     color: LinearRgba::WHITE,
            //     color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
            //     alpha_mode: AlphaMode::Blend,
            // })),
            MeshMaterial2d(color_materials.add(ColorMaterial::from_color(RED))),
            CollisionLayers::new(GameLayer::Groupset, GameLayer::Groupset.to_bits() | GameLayer::World.to_bits()),
            // GlobalTransform::default(),

            t.clone()
        )
    }


}