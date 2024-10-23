use avian2d::prelude::*;
use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, math::{dvec2, DVec2}, prelude::*, sprite::MaterialMesh2dBundle};

use crate::CustomMaterial;

use super::{components::BicycleWheel, plugin::BicyclePlugin};

impl BicyclePlugin {
    pub fn setup_bicycle(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut custom_materials: ResMut<Assets<CustomMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        let front_id = commands
            .spawn((
                BicycleWheel::Front,
                RigidBody::Dynamic,
                Collider::circle(BicycleWheel::size() as f64),
                CollisionMargin(1.0),
                Mass(1.0),
                Friction::new(0.95),
                Restitution::new(0.0),
                SweptCcd::default(),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(BicycleWheel::size())).into(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 10.0),
                        ..default()
                    },
    
                    material: custom_materials.add(CustomMaterial {
                        color: LinearRgba::WHITE,
                        color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                        alpha_mode: AlphaMode::Blend,
                    }),
                    ..default()
                },
            ))
            .id();
    
        let back_id = commands
            .spawn((
                BicycleWheel::Back,
                RigidBody::Dynamic,
                Collider::circle(BicycleWheel::size() as f64),
                CollisionMargin(1.0),
                Mass(1.0),
                Friction::new(0.95),
                Restitution::new(0.0),
                SweptCcd::default(),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(BicycleWheel::size())).into(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 10.0),
                        ..default()
                    },
                    material: custom_materials.add(CustomMaterial {
                        color: LinearRgba::WHITE,
                        color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                        alpha_mode: AlphaMode::Blend,
                    }),
                    ..default()
                },
            ))
            .id();
    
        let rear_hub = dvec2(-40.0, 0.0);
        let front_hub = dvec2(35.0, 0.0);
        let bottom_bracket = dvec2(0.0, 0.0);
        let seat_clamp = dvec2(-10.0, 20.0);
        let stem_clamp = dvec2(30.0, 20.0);
    
        let frame_points_all: Vec<DVec2> =
            vec![rear_hub, bottom_bracket, seat_clamp, stem_clamp, front_hub];
        let frame_points_all_indicies: Vec<[u32; 2]> =
            vec![[0, 1], [1, 2], [2, 0], [2, 3], [1, 3], [3, 4]];
    
        let frame_collider =
            Collider::convex_decomposition(frame_points_all, frame_points_all_indicies);
    
        let frame_id = commands
            .spawn((
                RigidBody::Dynamic,
                frame_collider,
                Sensor,
                MassPropertiesBundle {
                    mass: Mass(10.0),
                    ..default()
                },
            ))
            .id();
    
        let crank_collider = Collider::polyline(
            vec![
                bottom_bracket + 8.0 * DVec2::Y,
                bottom_bracket + 8.0 * DVec2::NEG_Y,
            ],
            vec![[0, 1]].into(),
        );
    
        let crank = commands
            .spawn((
                RigidBody::Dynamic,
                crank_collider,
                Sensor,
                MassPropertiesBundle {
                    mass: Mass(10.0),
                    ..default()
                },
            ))
            .id();
    
        commands.spawn(
            RevoluteJoint::new(frame_id, front_id)
                .with_local_anchor_1(front_hub)
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
        );
    
        commands.spawn(
            RevoluteJoint::new(frame_id, back_id)
                .with_local_anchor_1(rear_hub)
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
        );
    
        commands.spawn(
            RevoluteJoint::new(frame_id, crank)
                .with_local_anchor_1(bottom_bracket)
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
        );
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