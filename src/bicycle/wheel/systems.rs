use avian2d::prelude::*;
use bevy::{color::palettes::css::BLACK, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*};

use crate::{bicycle::{groupset::components::Cog, systems::{AttachmentPoint, GameLayer}}, CustomMaterial};

use super::{components::BicycleWheel, events::SpawnWheelEvent, plugin::WheelPlugin};

impl WheelPlugin {

    pub fn spawn_wheel(
        trigger: Trigger<SpawnWheelEvent>,
        mut commands: Commands,
        attachment_points: Query<(Entity, &AttachmentPoint, &Transform)>,
        cogs: Query<(Entity, &Cog)>,
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
            CollisionLayers::new(GameLayer::Wheels, GameLayer::World),
            DebugRender::default().with_collider_color(BLACK.into()),
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
            t.clone()
        )).id();
    
        let joint = commands.spawn((
            Name::new("Wheel Joint"),
            RevoluteJoint::new(attachment_point_ent, wheel)
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
        )).id();

        let (cassette_ent, _) = cogs.iter().find(|item| item.1 == &Cog::RearCassette).unwrap();

        if evt.wheel ==  BicycleWheel::Back {
            commands.spawn((
                Name::new("Wheel Joint"),

                // FixedJoint::new(cassette_ent, wheel)
                //     .with_angular_velocity_damping(0.0)
                //     .with_linear_velocity_damping(0.0),

                RevoluteJoint::new(cassette_ent, wheel)
                    .with_compliance(0.0)
                    .with_angular_velocity_damping(0.0)
                    .with_linear_velocity_damping(0.0)
            ));
        }
    
    }

    // pub fn spin_wheel(
    //     mut wheel_query: Query<(&BicycleWheel, &mut ExternalTorque), With<BicycleWheel>>,
    //     mut mouse_wheel_evt: EventReader<MouseWheel>,
    // ) {
    //     for &evt in mouse_wheel_evt.read() {
    //         match &evt.unit {
    //             MouseScrollUnit::Line => {
    //                 for (wheel, mut torque) in wheel_query.iter_mut() {
    //                     if let BicycleWheel::Back = wheel {
    //                         *torque = ExternalTorque::new(-2000000.0_f64 * evt.y as f64)
    //                             .with_persistence(true);
    //                         // ang_vel.0 += -10.0 as f64 * evt.y as f64;
    //                         println!("torque {}", torque.torque());
    //                     }
    //                 }
    //             }
    //             MouseScrollUnit::Pixel => {}
    //         }
    //     }
    // }
}

