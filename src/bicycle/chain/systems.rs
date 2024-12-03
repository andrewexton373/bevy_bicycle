use std::f32::consts::PI;

use avian2d::prelude::*;
use bevy::{math::vec3, prelude::*};

use super::plugin::ChainPlugin;

impl ChainPlugin {
    pub fn setup_chain(mut commands: Commands) {
        let chain_links = 20;
        let link_radius = 1.0;
        let theta = 2.0 * PI / chain_links as f32;
        let r = 30.0;

        let initial_link = commands
            .spawn((
                RigidBody::Dynamic,
                Collider::circle(link_radius),
                MassPropertiesBundle {
                    mass: Mass::new(0.01),
                    ..default()
                },
                Transform {
                    translation: vec3(f32::cos(0.0) * r, f32::sin(0.0) * r, 0.0),
                    ..default()
                },
            ))
            .id();

        let mut previous_link = initial_link;

        for n in 1..=chain_links {
            let current_link = commands
                .spawn((
                    RigidBody::Dynamic,
                    Collider::circle(link_radius),
                    MassPropertiesBundle {
                        mass: Mass::new(0.01),
                        ..default()
                    },
                    Transform {
                        translation: vec3(
                            f32::cos(n as f32 * theta) * r,
                            f32::sin(n as f32 * theta) * r,
                            0.0,
                        ),
                        ..default()
                    },
                ))
                .id();

            commands.spawn(
                DistanceJoint::new(previous_link, current_link)
                    // .with_local_anchor_2(V ector::Y * (link_radius * 2.0 + 1.0))
                    .with_rest_length(10.0)
                    .with_compliance(0.0000001),
            );

            // Form the Loop
            if n == chain_links {
                commands.spawn(
                    DistanceJoint::new(current_link, initial_link)
                        .with_rest_length(10.0)

                        // .with_local_anchor_2(Vector::Y * (link_radius * 2.0 + 1.0))
                        .with_compliance(0.0000001),
                );
            }

            previous_link = current_link;
        }

        let chain_id = commands.spawn(());
    }
}
