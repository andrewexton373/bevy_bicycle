use avian2d::{math::{Vector, PI}, parry::{na::geometry, shape::HeightField}, prelude::*};
use bevy::{asset::RenderAssetUsages, color::palettes::css::{GRAY, LIGHT_GREEN}, math::DVec2, prelude::*, render::{camera, mesh::{Indices, PrimitiveTopology}}};
use noise::{NoiseFn, Perlin};
use rand::RngCore;

use crate::camera::components::FollowCamera;

use super::plugin::WorldPlugin;

#[derive(Component, PartialEq)]
pub struct TerrainChunk(pub i128);

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

        let collider = WorldPlugin::generate_hilly_terrain_chunk(0, 0);

        commands.spawn((
            RigidBody::Static,
            collider,
            // DebugRender::all().with_mesh_visibility(true),
            Friction::new(0.95),
            Restitution::new(0.0),
            SweptCcd::default(),

            // Mesh2d(meshes.add(WorldPlugin::heightmap_to_bevy_mesh(collider.shape().as_heightfield().unwrap().heights(), Vec2::new(1.0, 1.0)))),
            // MeshMaterial2d(materials.add(ColorMaterial::from_color(LIGHT_GREEN))),
            Transform::from_xyz(-1000.0, -4050.0, 10.0),
        ));
    }

    const CHUNK_WIDTH: f32 = 2048.0;
    const WINDOW_SIZE: u128 = 16;


    pub fn generate_surrounding_terrain_chunks(
        mut commands: Commands,
        camera: Query<&Transform, With<FollowCamera>>,
        terrain_chunks: Query<&TerrainChunk>,
        mut seed: Local<Option<u32>>
    ) {

        if seed.is_none() {
            let mut rng = rand::thread_rng();
        // let seed: u32 = rng.next_u32();
            *seed = Some(rng.next_u32());
        }

        let camera_t = camera.single();

        // info!("Camera_t: {:?}", camera_t);

        let camera_t_x = camera_t.translation.x;
        let chunk_index = (camera_t_x / (WorldPlugin::CHUNK_WIDTH)) as i128;

        info!("Camera_t current chunk index: {:?}", chunk_index);
        info!("Seed: {:?}", seed);



        for index in chunk_index - Self::WINDOW_SIZE as i128 / 2 .. chunk_index + (Self::WINDOW_SIZE as i128 / 2) - 1 {
            // If the chunk doesn't exist
            if None == terrain_chunks.iter().find(|chunk| chunk.0 == chunk_index) {
                info!("Creating Chunk {:?}", chunk_index);

                let chunk_collider = WorldPlugin::generate_hilly_terrain_chunk(index, seed.unwrap());

                commands.spawn((
                    Name::new(format!("TerrainChunk({:?})", index)),
                    TerrainChunk(chunk_index),
                    RigidBody::Static,
                    chunk_collider,
                    // DebugRender::all().with_mesh_visibility(true),
                    Friction::new(0.95),
                    Restitution::new(0.0),
                    SweptCcd::default(),
        
                    // Mesh2d(meshes.add(WorldPlugin::heightmap_to_bevy_mesh(collider.shape().as_heightfield().unwrap().heights(), Vec2::new(1.0, 1.0)))),
                    // MeshMaterial2d(materials.add(ColorMaterial::from_color(LIGHT_GREEN))),
        
                    Transform::from_xyz(((index as f64 * Self::CHUNK_WIDTH as f64).floor() - (Self::CHUNK_WIDTH / 2.0) as f64) as f32, -4050.0, 10.0),
                ));

            } else {
                // info!("Chunk {:?} exists!", chunk_index)
            }       
        }  

    }


    pub fn generate_hilly_terrain_chunk(
        chunk_index:  i128,
        seed: u32
    ) -> Collider {

        // let mut rng = rand::thread_rng();
        // let seed: u32 = rng.next_u32();

        let perlin = Perlin::new(seed);
    
        let origin = DVec2::ZERO;
        let substep_count = 100;
        let substep_width = Self::CHUNK_WIDTH as f64 / substep_count as f64;

        let mut geometry = vec![];
        // geometry.push(origin);

        let sample = |x: f64| {1000.0 * (perlin.get([0.2 * x / Self::CHUNK_WIDTH as f64]) + 1.0)};

        // Sample Points on Function
        for i in 0..substep_count {

            let x = chunk_index as f64 * Self::CHUNK_WIDTH as f64 + substep_width * i as f64;
            let sample_point = sample(x);

            geometry.push(sample_point);
        }

        let heightfield_collider = Collider::heightfield(geometry.into_iter().collect(), Vector::new(Self::CHUNK_WIDTH as f64, 1.0));
        // let bottom_segment_collider= Collider::segment(origin, origin + DVec2::new(Self::CHUNK_WIDTH as f64, 0.0));
        // let left_segment_collider = Collider::segment(origin, origin + DVec2::new(0.0, sample(0.0)));
        // let right_segment_collider = Collider::segment(origin + DVec2::new(Self::CHUNK_WIDTH as f64, 0.0), origin + DVec2::new(Self::CHUNK_WIDTH as f64, sample(Self::CHUNK_WIDTH as f64)));

        let compound_collider = Collider::compound(vec![
            (Position::new(DVec2::new(Self::CHUNK_WIDTH as f64 / 2.0, 0.0)), Rotation::default(), heightfield_collider),
            // (Position::default(), Rotation::default(), bottom_segment_collider),
            // (Position::default(), Rotation::default(), left_segment_collider),
            // (Position::default(), Rotation::default(), right_segment_collider),
        ]);

        compound_collider
    }
}
