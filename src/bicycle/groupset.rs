use avian2d::prelude::*;
use bevy::{
    color::palettes::css::{GREEN, RED},
    ecs::system::{RunSystemOnce, SystemState},
    input::mouse::MouseWheel,
    prelude::*,
};

use crate::{
    bicycle::{
        frame::{BicycleFrame, FrameGeometry},
        systems::BicycleSystems,
        wheel::BicycleWheel,
    },
    GameLayer,
};

use bevy::{
    app::{Plugin, PostUpdate},
    prelude::{in_state, IntoSystemConfigs},
};
use std::f64::consts::PI;

use crate::GameState;

use bevy::prelude::Component;

#[derive(Component)]
pub struct Groupset;

#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub enum Cog {
    FrontChainring,
    RearCassette,
}

#[derive(Component, Clone, Copy)]
pub struct Radius(pub f32);

// Define a struct for a 2D point
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    // // Function to calculate the Euclidean distance between two points
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // Interpolate between two points, given a ratio (0.0 to 1.0).
    pub fn interpolate(&self, other: &Point, t: f64) -> Point {
        Point {
            x: self.x + t * (other.x - self.x),
            y: self.y + t * (other.y - self.y),
        }
    }
}

// Define a struct for a 2D disc (circle)
#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct Disc {
    pub center: Point,
    pub radius: f64,
}

impl Disc {
    // Function to generate points on a circle's boundary and return those points as a resulting simplified polygon
    pub fn simplify_disc_as_polygon(&self, num_vertices: usize) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..num_vertices {
            let angle = 2.0 * PI * (i as f64) / (num_vertices as f64);
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();
            points.push(Point { x, y });
        }
        points
    }
}

pub struct GroupsetPlugin;
impl Plugin for GroupsetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                update_chainring_size,
                update_cassette_size,
                turn_crank,
                limit_crank_rpm,
            )
                .chain()
                .after(PhysicsSet::Sync)
                .run_if(in_state(GameState::Ready)),
        )
        .init_resource::<ChainringRadius>()
        .init_resource::<CassetteRadius>();
    }
}
use bevy::prelude::Resource;

#[derive(Resource, PartialEq)]
pub struct ChainringRadius(pub f32);

impl Default for ChainringRadius {
    fn default() -> Self {
        ChainringRadius(5.0)
    }
}

#[derive(Resource, PartialEq)]
pub struct CassetteRadius(pub f32);

impl Default for CassetteRadius {
    fn default() -> Self {
        CassetteRadius(5.0)
    }
}

pub(crate) fn spawn_groupset(world: &mut World) {
    world
        .run_system_once_with(Cog::FrontChainring, spawn_component)
        .expect("Error Spawning Front Chainring");
    world
        .run_system_once_with(Cog::RearCassette, spawn_component)
        .expect("Error Spawning Rear Cassette");
}

fn spawn_component(In(cog): In<Cog>, world: &mut World) {
    let mut system_state: SystemState<(
        Commands,
        Query<(Entity, &BicycleFrame, &Transform)>,
        Query<(Entity, &BicycleWheel)>,
        Res<CassetteRadius>,
        Res<ChainringRadius>,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
    )> = SystemState::new(world);

    let (mut commands, frame, wheels, cassette_radius, chainring_radius, meshes, color_materials) =
        system_state.get_mut(world);

    let (frame_ent, frame, transform) = frame.single();

    match cog {
        Cog::FrontChainring => {
            let pos = *frame.geometry.get(&FrameGeometry::BottomBracket).unwrap();
            let front_chainring = commands
                .spawn(front_chainring(
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
                .spawn(rear_cassette(
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

fn update_chainring_size(
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

fn update_cassette_size(
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

fn front_chainring(
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
            GameLayer::Groupset.to_bits() | GameLayer::World.to_bits() | GameLayer::Chain.to_bits(),
        ),
        *t,
    )
}

fn rear_cassette(
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
            GameLayer::Groupset.to_bits() | GameLayer::World.to_bits() | GameLayer::Chain.to_bits(),
        ),
        *t,
    )
}

fn limit_crank_rpm(mut cogs: Query<(&Cog, &mut AngularVelocity), With<Cog>>) {
    for (cog, mut _ang_vel) in cogs.iter_mut() {
        if cog == &Cog::FrontChainring {
            let current_rpm = ang_vel_to_rpm(_ang_vel.0);
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

fn turn_crank(
    mut cogs: Query<(&Cog, &mut AngularVelocity, &mut ExternalTorque), With<Cog>>,
    _mouse_wheel_evt: EventReader<MouseWheel>,
) {
    for (cog, ang_vel, mut torque) in cogs.iter_mut() {
        if let Cog::FrontChainring = cog {
            torque.clear();
            if ang_vel_to_rpm(ang_vel.0).abs() > 90.0 {
                torque.clear();
            } else {
                torque.apply_torque(-2000.0_f64);
            }
        }
    }
}
