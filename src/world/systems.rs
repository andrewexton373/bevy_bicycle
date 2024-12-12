use avian2d::{math::Vector, parry::na::geometry, prelude::*};
use bevy::{color::palettes::css::{GRAY, LIGHT_GREEN}, math::DVec2, prelude::*};

use super::plugin::WorldPlugin;

impl WorldPlugin {
    pub fn setup_ground(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let width: f64 = 10000.0;
        let height: f64 = 300.0;

        // commands.spawn((
        //     RigidBody::Static,
        //     Collider::rectangle(width, height),
        //     Friction::new(0.95),
        //     Restitution::new(0.0),
        //     SweptCcd::default(),
        //     Mesh2d(meshes.add(Rectangle::new(width as f32, height as f32))),
        //     MeshMaterial2d(materials.add(ColorMaterial::from_color(GRAY))),
        //     Transform::from_xyz(0.0, -200.0, 10.0),
        // ));

        commands.spawn((
            RigidBody::Static,
            WorldPlugin::generate_hilly_terrain_chunk(),
            Friction::new(0.95),
            Restitution::new(0.0),
            SweptCcd::default(),
            // Mesh2d(meshes.add(Rectangle::new(width as f32, height as f32))),
            // MeshMaterial2d(materials.add(ColorMaterial::from_color(LIGHT_GREEN))),
            Transform::from_xyz(-100.0, -1050.0, 10.0),
        ));
    }


    pub fn generate_hilly_terrain_chunk(

    ) -> Collider {

        let origin = DVec2::new(0.0, 0.0);
        let chunk_width = 100000.0;
        let substep_count = 10;
        let substep_width = 1000.0 / 10 as f64;

        let mut geometry = vec![];
        // geometry.push(origin);

        let sample = |x: f64| {1000.0 + 1000.0 * x.sin()};

        // Sample Points on Function
        for i in 0..substep_count {

            let x = substep_width * i as f64;
            let sample_point = sample(x);

            geometry.push(sample_point);
        }

        // geometry.push(origin + DVec2::new(chunk_width, 0.0));

        // let collider = Collider::polyline(geometry, None);

        let heightfield_collider = Collider::heightfield(geometry.into_iter().collect(), Vector::new(chunk_width, 1.0));
        let bottom_segment_collider= Collider::segment(origin, origin + DVec2::new(chunk_width, 0.0));
        let left_segment_collider = Collider::segment(origin, origin + DVec2::new(0.0, sample(0.0)));
        let right_segment_collider = Collider::segment(origin + DVec2::new(chunk_width, 0.0), origin + DVec2::new(chunk_width, sample(chunk_width)));

        let compound_collider = Collider::compound(vec![
            (Position::new(DVec2::new(chunk_width / 2.0, 0.0)), Rotation::default(), heightfield_collider),
            (Position::default(), Rotation::default(), bottom_segment_collider),
            (Position::default(), Rotation::default(), left_segment_collider),
            (Position::default(), Rotation::default(), right_segment_collider),
        ]);



        compound_collider
    }
}
