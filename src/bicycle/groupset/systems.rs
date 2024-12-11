use avian2d::prelude::*;
use bevy::{color::palettes::css::{GREEN, RED, WHEAT}, ecs::entity, input::{keyboard::KeyboardInput, mouse::{MouseScrollUnit, MouseWheel}}, prelude::*, state::commands};

use crate::{bicycle::{components::{BicycleFrame, FrameGeometry}, groupset::events::SpawnAttachedEvent, systems::{GameLayer}, wheel::components::BicycleWheel}, CustomMaterial};

use super::{components::{Axle, Cog, Disc, Groupset, Point, Radius}, events::SpawnGroupsetEvent, plugin::GroupsetPlugin};

impl GroupsetPlugin {

    pub fn init_groupset(
        _: Trigger<SpawnGroupsetEvent>,
        mut commands: Commands
    ) {

        commands.trigger(SpawnAttachedEvent {
            cog: Cog::FrontChainring
        });

        commands.trigger(SpawnAttachedEvent {
            cog: Cog::RearCassette
        });

    }

    pub fn handle_spawn_component(
        trigger: Trigger<SpawnAttachedEvent>,
        mut commands: Commands,
        frame: Query<(Entity, &BicycleFrame)>,
        wheels: Query<(Entity, &BicycleWheel)>,

        meshes: ResMut<Assets<Mesh>>,
        color_materials: ResMut<Assets<ColorMaterial>>,
    ) {

        let cog = trigger.event().cog;
        let (frame_ent, frame) = frame.single();

        match cog {
            Cog::FrontChainring => {
                let pos = frame.gemometry.get(&FrameGeometry::BottomBracket).unwrap().as_dvec2();
                let front_chainring = commands.spawn(GroupsetPlugin::front_chainring(meshes, color_materials, &Position::from(pos))).id();
                
                commands.spawn(
                    (
                        Name::new("Bottom Bracket / Chainring Revolute Joint"),
                        RevoluteJoint::new(frame_ent, front_chainring)
                            .with_local_anchor_1(frame.gemometry.get(&FrameGeometry::BottomBracket).unwrap().as_dvec2())
                            .with_angular_velocity_damping(0.0)
                            .with_linear_velocity_damping(0.0)
                    )
                );
            },
            Cog::RearCassette => {
                let pos = frame.gemometry.get(&FrameGeometry::BottomBracket).unwrap().as_dvec2();

                let rear_cassette = commands.spawn(GroupsetPlugin::rear_cassette(meshes, color_materials, &Position::from(pos))).id();

                commands.spawn((
                    Name::new("Rear Wheel Fork / Cassette Revolute Joint"),
                    RevoluteJoint::new(frame_ent, rear_cassette)
                    .with_local_anchor_1(frame.gemometry.get(&FrameGeometry::RearHub).unwrap().as_dvec2())
                        .with_angular_velocity_damping(0.0)
                        .with_linear_velocity_damping(0.0)
                ));

                let (wheel_ent, wheel) = wheels.iter().find(|item| item.1 == &BicycleWheel::Back).unwrap();


                commands.spawn((
                    Name::new("Rear Wheel / Cassette Fixed Joint"),
                    FixedJoint::new(wheel_ent, rear_cassette)
                    // .with_local_anchor_1(frame.gemometry.get(&FrameGeometry::RearHub).unwrap().as_dvec2())
                        .with_angular_velocity_damping(0.0)
                        .with_linear_velocity_damping(0.0)
                ));
            },
        }

    }

    pub fn front_chainring(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
        t: &Position
    ) -> impl Bundle {
        let wheel_radius = Radius(5.0);
        (
            // Axle::FRONT,
            Cog::FrontChainring,
            Name::new("Front Chainring"),
            wheel_radius,
            RigidBody::Dynamic,
            Collider::circle(wheel_radius.0 as f64),
            CollisionMargin(0.1),
            Mass::new(1.0),
            Friction::new(1.0),
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
                            println!("TURN CRANK: ang_vel {}", ang_vel.0);
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
        t: &Position
    ) -> impl Bundle {

        let wheel_radius = Radius(10.0);

        (
            // Axle::REAR,
            Cog::RearCassette,
            Name::new("Rear Cassette"),
            wheel_radius,
            RigidBody::Dynamic,
            Collider::circle(wheel_radius.0 as f64),
            CollisionMargin(0.1),
            Mass::new(1.0),
            Friction::new(1.0),
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