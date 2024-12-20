use std::f64::MIN;

use avian2d::{math::Vector, prelude::*};
use bevy::{asset::RenderAssetUsages, color::palettes::css::{GREEN, LIGHT_GREEN, LIMEGREEN, RED, WHITE}, core_pipeline::core_3d::Opaque3d, math::DVec2, pbr::{wireframe::Wireframe, OpaqueRendererMethod}, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_resource::Face}};
use noise::{NoiseFn, Perlin};

use crate::camera::components::FollowCamera;

use super::{
    plugin::WorldTerrainPlugin,
    resources::{MaxTerrainChunkCount, TerrainSeed},
};

#[derive(Component)]
pub struct Terrain;

#[derive(Component, PartialEq)]
pub struct TerrainChunk(pub i128);

impl WorldTerrainPlugin {
    pub const CHUNK_WIDTH: f32 = 2048.0;

    pub fn x_pos_to_chunk_index(pos: f64) -> i128 {
        (pos / Self::CHUNK_WIDTH as f64).round() as i128
    }

    pub fn generate_surrounding_terrain_chunks(
        mut commands: Commands,
        camera: Query<&Transform, With<FollowCamera>>,
        terrain_chunks: Query<&TerrainChunk>,
        terrain: Query<(Entity, Option<&Children>), With<Terrain>>,
        terrain_seed: Res<TerrainSeed>,
        terrain_chunk_count: Res<MaxTerrainChunkCount>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mut terrain_id: Entity = Entity::PLACEHOLDER;

        if terrain.is_empty() {
            info!("Spawning Terrain Parent");
            terrain_id = commands
                .spawn((Terrain, Name::new("Terrain"), Transform::default(), InheritedVisibility::VISIBLE))
                .id();
        }

        if let Ok((terrain_id, terrain_chunks_query)) = terrain.get_single() {
            let camera_t = camera.single();

            // info!("Camera_t: {:?}", camera_t);

            let camera_t_x = camera_t.translation.x;
            let chunk_index = Self::x_pos_to_chunk_index(camera_t_x as f64);

            // info!("Camera_t current chunk index: {:?}", chunk_index);
            // info!("Seed: {:?}", terrain_seed.0);

            for index in chunk_index - terrain_chunk_count.0 as i128 / 2
                ..chunk_index + (terrain_chunk_count.0 as i128 / 2)
            {
                // If the chunk doesn't exist
                if !terrain_chunks.iter().any(|chunk| chunk.0 == index) {
                    info!("Creating Chunk {:?}", index);

                    let (chunk_collider, chunk_mesh) =
                        WorldTerrainPlugin::generate_hilly_terrain_chunk(index, terrain_seed.0);

                    commands.entity(terrain_id).with_child((
                        Name::new(format!("TerrainChunk({:?})", index)),
                        TerrainChunk(index),
                        RigidBody::Static,
                        chunk_collider,
                        Friction::new(0.95),
                        Restitution::new(0.0),
                        SweptCcd::default(),
                        Mesh3d(meshes.add(chunk_mesh)),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: LIGHT_GREEN.into(),
                            unlit: true,
                            ..Default::default()
                        })),
                        // Wireframe,
                        Transform::from_xyz((index as f32).round() * Self::CHUNK_WIDTH, 0.0, 10.0),
                    ));
                }
            }
        }
    }

    pub fn terrain_height_sample(x_pos: f64, seed: u32) -> f64 {
        let perlin = Perlin::new(seed);
        100.0 * (perlin.get([0.0001 * x_pos]) + 1.0) / Self::CHUNK_WIDTH as f64
    }

    pub fn generate_hilly_terrain_chunk(chunk_index: i128, seed: u32) -> (Collider, Mesh) {
        let substep_count: i32 = 20;
        let substep_width = Self::CHUNK_WIDTH as f64 / substep_count as f64;

        let mut geometry = vec![];

        // Sample Points via Terrain Generation Function
        for i in 0..=substep_count {
            let x =
                (chunk_index as f64 * Self::CHUNK_WIDTH as f64).floor() + substep_width * i as f64;
            let sample_point = Self::terrain_height_sample(x, seed);

            geometry.push(sample_point);
        }

        let heightfield_collider = Collider::heightfield(
            geometry.clone().into_iter().collect(),
            Vector::splat(Self::CHUNK_WIDTH as f64),
        );

        let mut verticies: Vec<[f32; 3]> = vec![];
        let mut indicies = vec![];
        let mut normals = vec![];
        let geometry_2 = geometry.clone();

        info!("COUNT: {}", geometry_2.iter().count());

        let mut sample_heights = geometry_2.iter().enumerate().peekable();
        while let Some((i, height)) = sample_heights.next() {

            info!("HIT! {} {}", i, height);
            
            let substep_width = substep_width as f32;
            let height = *height as f32;

            verticies.push([(i as f32 * substep_width) - Self::CHUNK_WIDTH / 2.0, -1000.0, 0.0]); // Vertex 0
            verticies.push([(i as f32 * substep_width) - Self::CHUNK_WIDTH / 2.0, (height * Self::CHUNK_WIDTH) - 1.0, 0.0]); 

            normals.push([0.,0.,1.]);
            normals.push([0.,0.,1.]);

        }

        for step in 0..substep_count {
            let i = (step * 2) as u32;
            info!("STEP: {}", step);

            // First Triangle
            indicies.push(i);
            indicies.push(i + 2);
            indicies.push(i + 1);

            // Second Triangle
            indicies.push(i + 1);            
            indicies.push(i + 2);            
            indicies.push(i + 3);            
        }

        info!("Vertex Count: {}", verticies.iter().count());
        info!("Index Count: {}", indicies.iter().count());
        



        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            verticies
        )
        // Assign a UV coordinate to each vertex.
        // .with_inserted_attribute(
        //     Mesh::ATTRIBUTE_UV_0,
        //     vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]]
        // )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_inserted_indices(Indices::U32(indicies));

        (Collider::compound(vec![(
            Position::new(DVec2::ZERO),
            Rotation::default(),
            heightfield_collider,
        )]),
            mesh
        )
    }

    pub fn remove_chunks_outside_viewport(
        mut commands: Commands,
        camera_viewport: Query<(&Camera, &GlobalTransform), With<FollowCamera>>,
        terrain_chunks: Query<(Entity, &TerrainChunk), With<TerrainChunk>>,
        terrain: Query<Entity, With<Terrain>>,
        terrain_chunk_count: Res<MaxTerrainChunkCount>,
    ) {
        let (camera, camera_gt) = camera_viewport.single();

        // Get viewport bounds in worldspace
        let left_bound = camera
            .ndc_to_world(camera_gt, Vec3::new(-1.0, 0.0, 0.0))
            .unwrap();
        let right_bound = camera
            .ndc_to_world(camera_gt, Vec3::new(1.0, 0.0, 0.0))
            .unwrap();

        let current_chunk_index = Self::x_pos_to_chunk_index(camera_gt.translation().x as f64);
        let i_min = current_chunk_index.wrapping_sub(terrain_chunk_count.0 as i128 / 2);
        let i_max = current_chunk_index.wrapping_add(terrain_chunk_count.0 as i128 / 2);

        //info!(
        //    "current: {}, i_min: {}, i_max: {}",
        //    current_chunk_index, i_min, i_max
        //);

        // Filter Invalid sectors to despawn
        let invalid_sectors: Vec<(Entity, &TerrainChunk)> = terrain_chunks
            .iter()
            .filter(|(_, chunk)| chunk.0 < i_min || chunk.0 > i_max)
            .collect();

        if let Ok(terrain_id) = terrain.get_single() {
            // Despawn each invalid sector
            for (invalid_entity, chunk) in invalid_sectors {
                info!("Removing Terrain Chunk: {:?}", chunk.0);

                commands
                    .entity(terrain_id)
                    .remove_children(&[invalid_entity]);
                commands.entity(invalid_entity).despawn_recursive();
            }
        }
    }
}
