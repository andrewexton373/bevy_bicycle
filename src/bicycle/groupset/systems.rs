use avian2d::{parry::na::clamp, prelude::*};
use bevy::{
    color::palettes::css::{GREEN, RED},
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::bicycle::{
    chain::events::ResetChainEvent,
    components::{BicycleFrame, FrameGeometry},
    groupset::events::SpawnAttachedEvent,
    systems::GameLayer,
    wheel::components::BicycleWheel,
};

use super::{
    components::{Cog, Radius},
    events::SpawnGroupsetEvent,
    plugin::GroupsetPlugin,
    resources::{CassetteRadius, ChainringRadius},
};

impl GroupsetPlugin {
    pub fn init_groupset(_: Trigger<SpawnGroupsetEvent>, mut commands: Commands) {
        commands.trigger(SpawnAttachedEvent {
            cog: Cog::FrontChainring,
        });

        commands.trigger(SpawnAttachedEvent {
            cog: Cog::RearCassette,
        });

        // commands.trigger(ResetChainEvent);
    }

    pub fn update_chainring_size(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        chainring_radius: Res<ChainringRadius>,
        mut chainring: Query<(Entity, &Cog)>,
    ) {
        if chainring_radius.is_changed() {
            if let Some((ent, _)) = chainring
                .iter_mut()
                .find(|item| item.1 == &Cog::FrontChainring)
            {
                commands.entity(ent).insert(Radius(chainring_radius.0));
                commands
                    .entity(ent)
                    .insert(Collider::circle(chainring_radius.0 as f64));
                commands
                    .entity(ent)
                    .insert(Mesh2d(meshes.add(Circle::new(chainring_radius.0))));

                commands.trigger(ResetChainEvent);
            }
        }
    }

    pub fn update_cassette_size(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        cassette_radius: Res<CassetteRadius>,
        mut cogs: Query<(Entity, &Cog)>,
    ) {
        if cassette_radius.is_changed() {
            if let Some((ent, _)) = cogs.iter_mut().find(|item| item.1 == &Cog::RearCassette) {
                commands.entity(ent).insert(Radius(cassette_radius.0));
                commands
                    .entity(ent)
                    .insert(Collider::circle(cassette_radius.0 as f64));
                commands
                    .entity(ent)
                    .insert(Mesh2d(meshes.add(Circle::new(cassette_radius.0))));
                commands.trigger(ResetChainEvent);
            }
        }
    }

    pub fn handle_spawn_component(
        trigger: Trigger<SpawnAttachedEvent>,
        mut commands: Commands,
        frame: Query<(Entity, &BicycleFrame)>,
        wheels: Query<(Entity, &BicycleWheel)>,
        cassette_radius: Res<CassetteRadius>,
        chainring_radius: Res<ChainringRadius>,
        meshes: ResMut<Assets<Mesh>>,
        color_materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let cog = trigger.event().cog;
        let (frame_ent, frame) = frame.single();

        match cog {
            Cog::FrontChainring => {
                let pos = frame
                    .gemometry
                    .get(&FrameGeometry::BottomBracket)
                    .unwrap()
                    .as_dvec2();
                let front_chainring = commands
                    .spawn(GroupsetPlugin::front_chainring(
                        meshes,
                        color_materials,
                        chainring_radius,
                        &Position::from(pos),
                    ))
                    .id();

                commands.spawn((
                    Name::new("Bottom Bracket / Chainring Revolute Joint"),
                    RevoluteJoint::new(frame_ent, front_chainring)
                        .with_local_anchor_1(
                            frame
                                .gemometry
                                .get(&FrameGeometry::BottomBracket)
                                .unwrap()
                                .as_dvec2(),
                        )
                        .with_angular_velocity_damping(0.0)
                        .with_linear_velocity_damping(0.0),
                ));
            }
            Cog::RearCassette => {
                let pos = frame
                    .gemometry
                    .get(&FrameGeometry::BottomBracket)
                    .unwrap()
                    .as_dvec2();

                let rear_cassette = commands
                    .spawn(GroupsetPlugin::rear_cassette(
                        meshes,
                        color_materials,
                        cassette_radius,
                        &Position::from(pos),
                    ))
                    .id();

                commands.spawn((
                    Name::new("Rear Wheel Fork / Cassette Revolute Joint"),
                    RevoluteJoint::new(frame_ent, rear_cassette)
                        .with_local_anchor_1(
                            frame
                                .gemometry
                                .get(&FrameGeometry::RearHub)
                                .unwrap()
                                .as_dvec2(),
                        )
                        .with_angular_velocity_damping(0.0)
                        .with_linear_velocity_damping(0.0),
                ));

                let (wheel_ent, wheel) = wheels
                    .iter()
                    .find(|item| item.1 == &BicycleWheel::Back)
                    .unwrap();

                commands.spawn((
                    Name::new("Rear Wheel / Cassette Fixed Joint"),
                    FixedJoint::new(wheel_ent, rear_cassette)
                        // .with_local_anchor_1(frame.gemometry.get(&FrameGeometry::RearHub).unwrap().as_dvec2())
                        .with_angular_velocity_damping(0.0)
                        .with_linear_velocity_damping(0.0),
                ));
            }
        }
    }

    pub fn front_chainring(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
        chainring_radius: Res<ChainringRadius>,
        t: &Position,
    ) -> impl Bundle {
        let wheel_radius = Radius(chainring_radius.0);
        (
            // Axle::FRONT,
            Cog::FrontChainring,
            Name::new("Front Chainring"),
            wheel_radius,
            RigidBody::Dynamic,
            Collider::circle(wheel_radius.0 as f64),
            CollisionMargin(1.0),
            AngularVelocity::default(),
            Mass::new(1.0),
            Friction::new(1.0).with_combine_rule(CoefficientCombine::Max),
            Restitution::new(0.0),
            // SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
            Mesh2d(meshes.add(Circle::new(wheel_radius.0))),
            // MeshMaterial2d(custom_materials.add(CustomMaterial {
            //     color: LinearRgba::WHITE,
            //     color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
            //     alpha_mode: AlphaMode::Blend,
            // })),
            MeshMaterial2d(color_materials.add(ColorMaterial::from_color(GREEN))),
            CollisionLayers::new(
                GameLayer::Groupset,
                GameLayer::Groupset.to_bits()
                    | GameLayer::World.to_bits()
                    | GameLayer::Chain.to_bits(),
            ),
            // GlobalTransform::default(),
            *t,
        )
    }

    pub fn limit_crank_rpm(mut cogs: Query<(&Cog, &mut AngularVelocity), With<Cog>>) {
        for (cog, mut ang_vel) in cogs.iter_mut() {
            if cog == &Cog::FrontChainring {}
        }
    }

    pub fn ang_vel_to_rpm(ang_vel: f64) -> f64 {
        -ang_vel * 60.0 / (2.0 * std::f64::consts::PI)
    }

    pub fn rpm_to_ang_vel(rpm: f64) -> f64 {
        rpm / 60.0 * (2.0 * std::f64::consts::PI)
    }

    pub fn turn_crank(
        mut cogs: Query<(&Cog, &mut AngularVelocity, &mut ExternalTorque), With<Cog>>,
        mut mouse_wheel_evt: EventReader<MouseWheel>,
    ) {
        for (cog, mut ang_vel, mut torque) in cogs.iter_mut() {
            if let Cog::FrontChainring = cog {
                // torque.clear();

                // for &evt in mouse_wheel_evt.read() {
                //     match &evt.unit {
                //         MouseScrollUnit::Line => {
                //             // ang_vel.0 += -0.1_f64 * (evt.y as f64);
                //             println!("evt {:?}", evt.y);

                //             // if GroupsetPlugin::ang_vel_to_rpm(ang_vel.0).abs() > 90.0 {
                //                 torque.apply_torque(8000.0 * (evt.y as f64));
                //                 // *torque = *torque.with_persistence(true).apply_torque(200000.0 * (evt.y as f64));
                //                 println!("Torqeing: {:?}", torque);
                //             // }

                //             // ang_vel.0 += -1.0 as f64 * evt.y as f64;
                //             println!("TURN CRANK: ang_vel {}", ang_vel.0);
                //         }
                //         MouseScrollUnit::Pixel => {}
                //     }
                // }

                if GroupsetPlugin::ang_vel_to_rpm(ang_vel.0).abs() > 90.0 {
                    torque.clear();
                    // println!("MAXRPM!");
                } else {
                    torque.apply_torque(-8000.0 as f64);
                }
            }
        }
    }

    pub fn rear_cassette(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
        cassette_radius: Res<CassetteRadius>,

        t: &Position,
    ) -> impl Bundle {
        let wheel_radius = Radius(cassette_radius.0);

        (
            // Axle::REAR,
            Cog::RearCassette,
            Name::new("Rear Cassette"),
            wheel_radius,
            RigidBody::Dynamic,
            Collider::circle(wheel_radius.0 as f64),
            CollisionMargin(1.0),
            Mass::new(0.2),
            Friction::new(1.0).with_combine_rule(CoefficientCombine::Max),
            Restitution::new(0.0),
            // SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
            Mesh2d(meshes.add(Circle::new(wheel_radius.0))),
            // MeshMaterial2d(custom_materials.add(CustomMaterial {
            //     color: LinearRgba::WHITE,
            //     color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
            //     alpha_mode: AlphaMode::Blend,
            // })),
            MeshMaterial2d(color_materials.add(ColorMaterial::from_color(RED))),
            CollisionLayers::new(
                GameLayer::Groupset,
                GameLayer::Groupset.to_bits()
                    | GameLayer::World.to_bits()
                    | GameLayer::Chain.to_bits(),
            ),
            // GlobalTransform::default(),
            *t,
        )
    }
}
