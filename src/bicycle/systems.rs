use avian2d::prelude::*;
use bevy::{
    ecs::system::{RunSystemOnce, SystemState}, input::mouse::{MouseScrollUnit, MouseWheel}, math::{dvec2, DVec2}, prelude::*, sprite::MaterialMesh2dBundle, state::commands
};

use crate::CustomMaterial;

use super::{components::BicycleWheel, plugin::BicyclePlugin};

pub struct AttachmentPoints {
    frame_id: Entity,
    bottom_bracket: DVec2,
    front_hub: DVec2,
    rear_hub: DVec2
}

impl BicyclePlugin {

    pub fn spawn_wheel(
        In(wheel):In<BicycleWheel>,
        world: &mut World,
        params: &mut SystemState<(
            Commands,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<CustomMaterial>>,
            Res<AssetServer>,
        )>,
    ) -> Entity {

        let id = {
            let (mut commands, mut meshes, mut custom_materials, mut asset_server) =
            params.get_mut(world);

            commands
            .spawn((
                BicycleWheel::Front,
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
                })).id()
        };

        params.apply(world);

        id
   
    }

    

    pub fn spawn_frame(
        world: &mut World,
        params: &mut SystemState<(
            Commands,
        )>,
    ) -> AttachmentPoints {
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

        let frame_id = world
            .spawn((
                RigidBody::Dynamic,
                frame_collider,
                Sensor,
                MassPropertiesBundle {
                    mass: Mass::new(10.0),
                    ..default()
                },
            ))
            .id();

        return AttachmentPoints {
            frame_id,
            front_hub,
            bottom_bracket,
            rear_hub
        }
    }
    
    pub fn spawn_crank(
        In(attachment_point): In<DVec2>,
        world: &mut World,
    ) -> Entity {
        
        let crank_collider = Collider::polyline(
            vec![
                attachment_point + 8.0 * DVec2::Y,
                attachment_point + 8.0 * DVec2::NEG_Y,
            ],
            vec![[0, 1]].into(),
        );

        world
            .spawn((
                RigidBody::Dynamic,
                crank_collider,
                Sensor,
                MassPropertiesBundle {
                    mass: Mass::new(10.0),
                    ..default()
                },
            ))
            .id()
    }
    
    pub fn setup_bicycle(
        world: &mut World,
    ) {

        let front_id = world.run_system_once_with(BicycleWheel::Front, BicyclePlugin::spawn_wheel).unwrap();
        let back_id = world.run_system_once_with(BicycleWheel::Back, BicyclePlugin::spawn_wheel).unwrap();

        let attachment_points = world.run_system_once(BicyclePlugin::spawn_frame).unwrap();

        let crank = world.run_system_once_with(attachment_points.bottom_bracket, BicyclePlugin::spawn_crank).unwrap();

        world.spawn(
            RevoluteJoint::new(attachment_points.frame_id, front_id)
                .with_local_anchor_1(attachment_points.front_hub)
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
        );

        world.spawn(
            RevoluteJoint::new(attachment_points.frame_id, back_id)
                .with_local_anchor_1(attachment_points.rear_hub)
                .with_compliance(0.0)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(0.0),
        );

        world.spawn(
            RevoluteJoint::new(attachment_points.frame_id, crank)
                .with_local_anchor_1(attachment_points.bottom_bracket)
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
