use avian2d::prelude::*;
use bevy::{
    color::palettes::css::BLACK,
    prelude::*,
};

use crate::{
    bicycle::{
        components::{BicycleFrame, FrameGeometry},
        systems::GameLayer,
    },
    CustomMaterial,
};

use super::{components::BicycleWheel, events::SpawnWheelEvent, plugin::WheelPlugin};

impl WheelPlugin {
    pub fn spawn_wheel(
        trigger: Trigger<SpawnWheelEvent>,
        mut commands: Commands,
        frame: Query<(Entity, &BicycleFrame)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut custom_materials: ResMut<Assets<CustomMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        let evt = trigger.event();

        let (frame_ent, frame) = frame.single();

        let mounting_point = match evt.wheel {
            BicycleWheel::Front => frame
                .gemometry
                .get_key_value(&FrameGeometry::FrontHub)
                .unwrap(),
            BicycleWheel::Back => frame
                .gemometry
                .get_key_value(&FrameGeometry::RearHub)
                .unwrap(),
        };

        let wheel = commands
            .spawn((
                evt.wheel,
                Name::new("Wheel"),
                RigidBody::Dynamic,
                Collider::circle(BicycleWheel::size() as f64),
                CollisionLayers::new(GameLayer::Wheels, GameLayer::World),
                DebugRender::default().with_collider_color(BLACK.into()),
                CollisionMargin(1.0),
                Mass::new(0.001),
                Friction::new(1.0),
                Restitution::new(0.001),
                SweptCcd::default(),
                Mesh2d(meshes.add(Circle::new(BicycleWheel::size()))),
                MeshMaterial2d(custom_materials.add(CustomMaterial {
                    color: LinearRgba::WHITE,
                    color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                    alpha_mode: AlphaMode::Blend,
                })),
                Position::from(mounting_point.1.as_dvec2()),
            ))
            .id();

        commands.spawn((
            Name::new("Wheel Joint"),
            RevoluteJoint::new(frame_ent, wheel)
                .with_local_anchor_1(mounting_point.1.as_dvec2())
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
        ));
    }
}
