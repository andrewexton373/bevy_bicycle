use avian2d::prelude::*;
use bevy::{color::palettes::css::{RED, WHEAT}, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*};

use crate::CustomMaterial;

use super::{components::{Axle, Disc, Point}, plugin::GroupsetPlugin};

impl GroupsetPlugin {

    pub fn spawn_front_axle(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        // mut custom_materials: ResMut<Assets<ColorMaterial>>,
        mut custom_materials: ResMut<Assets<CustomMaterial>>,
        asset_server: Res<AssetServer>,


    ) {
        let wheel_size = 50.0;

        commands.spawn((
            Axle::FRONT,
            Disc {
                center: Point {x: 0.0, y: 0.0},
                radius: 50.0
            },
            RigidBody::Kinematic,
            Collider::circle(wheel_size as f64),
            CollisionMargin(1.0),
            Mass::new(1.0),
            Friction::new(0.99),
            Restitution::new(0.0),
            SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
            Mesh2d(meshes.add(Circle::new(wheel_size)).into()),
            // MeshMaterial2d(custom_materials.add(ColorMaterial::from_color(RED))),
            MeshMaterial2d(custom_materials.add(CustomMaterial {
                color: LinearRgba::WHITE,
                color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                alpha_mode: AlphaMode::Blend,
            })),
            Transform {
                translation: Vec3::new(-100.0, 200.0, 10.0),
                ..default()
            }
        ));
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

    pub fn spawn_back_axle(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut custom_materials: ResMut<Assets<ColorMaterial>>,
    ) {

        let wheel_size = 30.0;

        commands.spawn((
            Axle::FRONT,
            Disc {
                center: Point {x: 0.0, y: 0.0},
                radius: 30.0
            },
            RigidBody::Kinematic,
            Collider::circle(wheel_size as f64),
            CollisionMargin(1.0),
            Mass::new(1.0),
            Friction::new(0.95),
            Restitution::new(0.0),
            SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
            Mesh2d(meshes.add(Circle::new(wheel_size)).into()),
            MeshMaterial2d(custom_materials.add(ColorMaterial::from_color(RED))),
            Transform {
                translation: Vec3::new(100.0, 200.0, 10.0),
                ..default()
            }
        ));
    }


}