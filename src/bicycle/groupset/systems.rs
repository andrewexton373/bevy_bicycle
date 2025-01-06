use avian2d::{parry::na::clamp, prelude::*};
use bevy::{
    color::palettes::css::{GREEN, RED},
    ecs::system::{RunSystemOnce, SystemState},
    input::mouse::MouseWheel,
    prelude::*,
};

use crate::bicycle::{
    components::{BicycleFrame, FrameGeometry},
    systems::{BicycleSystems, GameLayer},
    wheel::components::BicycleWheel,
};

use super::{
    components::{Cog, Radius},
    plugin::GroupsetPlugin,
    resources::{CassetteRadius, ChainringRadius},
};

impl GroupsetPlugin {
    pub(crate) fn spawn_groupset(world: &mut World) {
        world
            .run_system_once_with(Cog::FrontChainring, Self::spawn_component)
            .expect("Error Spawning Front Chainring");
        world
            .run_system_once_with(Cog::RearCassette, Self::spawn_component)
            .expect("Error Spawning Rear Cassette");
    }

    pub(crate) fn spawn_component(In(cog): In<Cog>, world: &mut World) {
        let mut system_state: SystemState<(
            Commands,
            Query<(Entity, &BicycleFrame, &Transform)>,
            Query<(Entity, &BicycleWheel)>,
            Res<CassetteRadius>,
            Res<ChainringRadius>,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<StandardMaterial>>,
        )> = SystemState::new(world);

        let (
            mut commands,
            frame,
            wheels,
            cassette_radius,
            chainring_radius,
            meshes,
            color_materials,
        ) = system_state.get_mut(world);

        let (frame_ent, frame, transform) = frame.single();

        match cog {
            Cog::FrontChainring => {
                let pos = *frame.geometry.get(&FrameGeometry::BottomBracket).unwrap();
                let front_chainring = commands
                    .spawn(GroupsetPlugin::front_chainring(
                        meshes,
                        color_materials,
                        chainring_radius,
                        &Position::from(pos + transform.translation.truncate().as_dvec2()),
                    ))
                    .id();

                commands.spawn((
                    Name::new("Bottom Bracket / Chainring Revolute Joint"),
                    RevoluteJoint::new(frame_ent, front_chainring)
                        .with_local_anchor_1(
                            *frame.geometry.get(&FrameGeometry::BottomBracket).unwrap(),
                        )
                        .with_compliance(0.00001)
                        .with_angular_velocity_damping(0.0001)
                        .with_linear_velocity_damping(10.0),
                ));
            }
            Cog::RearCassette => {
                let pos = *frame.geometry.get(&FrameGeometry::BottomBracket).unwrap();

                let rear_cassette = commands
                    .spawn(GroupsetPlugin::rear_cassette(
                        meshes,
                        color_materials,
                        cassette_radius,
                        &Position::from(pos + transform.translation.truncate().as_dvec2()),
                    ))
                    .id();

                commands.spawn((
                    Name::new("Rear Wheel Fork / Cassette Revolute Joint"),
                    RevoluteJoint::new(frame_ent, rear_cassette)
                        .with_local_anchor_1(*frame.geometry.get(&FrameGeometry::RearHub).unwrap())
                        .with_compliance(0.00001)
                        .with_angular_velocity_damping(0.0001)
                        .with_linear_velocity_damping(10.0),
                ));

                let (wheel_ent, _wheel) = wheels
                    .iter()
                    .find(|item| item.1 == &BicycleWheel::Back)
                    .unwrap();

                commands.spawn((
                    Name::new("Rear Wheel / Cassette Fixed Joint"),
                    FixedJoint::new(wheel_ent, rear_cassette),
                ));
            }
        }
        system_state.apply(world);
    }

    pub fn update_chainring_size(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        chainring_radius: Res<ChainringRadius>,
        mut chainring: Query<(Entity, &Cog)>,
        systems: Res<BicycleSystems>,
    ) {
        if chainring_radius.is_changed() && !chainring_radius.is_added() {
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
                    .insert(Mesh3d(meshes.add(Circle::new(chainring_radius.0))));

                commands.run_system(systems.0["spawn_chain"]);
            }
        }
    }

    pub fn update_cassette_size(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        cassette_radius: Res<CassetteRadius>,
        mut cogs: Query<(Entity, &Cog)>,
        systems: Res<BicycleSystems>,
    ) {
        if cassette_radius.is_changed() && !cassette_radius.is_added() {
            if let Some((ent, _)) = cogs.iter_mut().find(|item| item.1 == &Cog::RearCassette) {
                commands.entity(ent).insert(Radius(cassette_radius.0));
                commands
                    .entity(ent)
                    .insert(Collider::circle(cassette_radius.0 as f64));
                commands
                    .entity(ent)
                    .insert(Mesh3d(meshes.add(Circle::new(cassette_radius.0))));

                commands.run_system(systems.0["spawn_chain"]);
            }
        }
    }

    pub fn front_chainring(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<StandardMaterial>>,
        chainring_radius: Res<ChainringRadius>,
        t: &Position,
    ) -> impl Bundle {
        let wheel_radius = Radius(chainring_radius.0);
        (
            Cog::FrontChainring,
            Name::new("Front Chainring"),
            wheel_radius,
            RigidBody::Dynamic,
            Collider::circle(wheel_radius.0 as f64),
            CollisionMargin(1.0),
            AngularVelocity::default(),
            Mass(1.0),
            Friction::new(1.0).with_combine_rule(CoefficientCombine::Max),
            Restitution::new(0.0),
            Mesh3d(meshes.add(Circle::new(wheel_radius.0))),
            MeshMaterial3d(color_materials.add(StandardMaterial::from_color(GREEN))),
            CollisionLayers::new(
                GameLayer::Groupset,
                GameLayer::Groupset.to_bits()
                    | GameLayer::World.to_bits()
                    | GameLayer::Chain.to_bits(),
            ),
            *t,
        )
    }

    pub fn rear_cassette(
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<StandardMaterial>>,
        cassette_radius: Res<CassetteRadius>,
        t: &Position,
    ) -> impl Bundle {
        let wheel_radius = Radius(cassette_radius.0);
        (
            Cog::RearCassette,
            Name::new("Rear Cassette"),
            wheel_radius,
            RigidBody::Dynamic,
            Collider::circle(wheel_radius.0 as f64),
            CollisionMargin(1.0),
            Mass(0.2),
            Friction::new(1.0).with_combine_rule(CoefficientCombine::Max),
            Restitution::new(0.0),
            Mesh3d(meshes.add(Circle::new(wheel_radius.0))),
            MeshMaterial3d(color_materials.add(StandardMaterial::from_color(RED))),
            CollisionLayers::new(
                GameLayer::Groupset,
                GameLayer::Groupset.to_bits()
                    | GameLayer::World.to_bits()
                    | GameLayer::Chain.to_bits(),
            ),
            *t,
        )
    }

    pub fn limit_crank_rpm(mut cogs: Query<(&Cog, &mut AngularVelocity), With<Cog>>) {
        for (cog, mut _ang_vel) in cogs.iter_mut() {
            if cog == &Cog::FrontChainring {
                let current_rpm = Self::ang_vel_to_rpm(_ang_vel.0);
                let clamped = current_rpm.clamp(-90.0, 90.0);

                // info!("Current RPM: {}", clamped);
                // _ang_vel.0 = Self::rpm_to_ang_vel(clamped); // _ang_vel.0.clamp(, max)
            }
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
        _mouse_wheel_evt: EventReader<MouseWheel>,
    ) {
        for (cog, ang_vel, mut torque) in cogs.iter_mut() {
            if let Cog::FrontChainring = cog {
                torque.clear();
                if GroupsetPlugin::ang_vel_to_rpm(ang_vel.0).abs() > 90.0 {
                    torque.clear();
                } else {
                    torque.apply_torque(-2000.0_f64);
                }
            }
        }
    }
}
