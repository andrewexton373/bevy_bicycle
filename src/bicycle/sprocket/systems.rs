use avian2d::prelude::*;
use bevy::prelude::*;

use crate::bicycle::sprocket::components::{Sprocket, SprocketOptions};

use super::plugin::SprocketPlugin;

impl SprocketPlugin {
    pub fn setup_sproket(mut commands: Commands) {
        let sproket = Sprocket::new(SprocketOptions {
            size: 3.0,
            teeth: 32,
        });
        let geometry = sproket.get_geometry();
        println!("GEO: {:?}", geometry);
        let collider = Collider::polyline(geometry, None);

        let id = commands
            .spawn((
                RigidBody::Static,
                collider,
                Sensor,
                MassPropertiesBundle {
                    mass: Mass(10.0),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(0.0, 100.0, 0.0),
                    ..default()
                }
            ))
            .id();
    }
}
