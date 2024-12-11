use avian2d::prelude::*;
use bevy::{color::palettes::css::GRAY, prelude::*, sprite::MaterialMesh2dBundle};

use super::plugin::WorldPlugin;

impl WorldPlugin {
    pub fn setup_ground(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let width: f64 = 10000.0;
        let height: f64 = 300.0;

        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(width, height),
            Friction::new(0.95),
            Restitution::new(0.0),
            SweptCcd::default(),
            Mesh2d(
                meshes
                    .add(Rectangle::new(width as f32, height as f32))
                    .into(),
            ),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(GRAY))),
            Transform::from_xyz(0.0, -200.0, 10.0),
        ));
    }
}
