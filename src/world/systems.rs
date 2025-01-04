use avian2d::{math::Vector, prelude::*};
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::LIGHT_GREEN,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use noise::{NoiseFn, Perlin};

use crate::{bicycle::systems::GameLayer, camera::components::FollowCamera};

use super::{
    components::{Terrain, TerrainChunk},
    plugin::WorldTerrainPlugin,
    resources::{MaxTerrainChunkCount, TerrainSeed},
};

impl WorldTerrainPlugin {
    pub const CHUNK_WIDTH: f32 = 2048.0;
    pub const SUBSTEP_COUNT: u32 = 20;

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
                .spawn((
                    Terrain,
                    Name::new("Terrain"),
                    Transform::default(),
                    InheritedVisibility::VISIBLE,
                ))
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
                        CollisionLayers::new(
                            [GameLayer::World],
                            [GameLayer::Wheels, GameLayer::Frame],
                        ),
                        RigidBody::Static,
                        CollisionMargin(1.0),
                        chunk_collider,
                        Friction::new(0.95),
                        Restitution::new(0.0),
                        // SweptCcd::default(),
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

    fn substep_width() -> f64 {
        Self::CHUNK_WIDTH as f64 / Self::SUBSTEP_COUNT as f64
    }

    pub fn generate_hilly_terrain_chunk(chunk_index: i128, seed: u32) -> (Collider, Mesh) {
        let mut terrain_height_samples = vec![];

        // Sample Points via Terrain Generation Function
        for i in 0..=Self::SUBSTEP_COUNT {
            let x = (chunk_index as f64 * Self::CHUNK_WIDTH as f64).floor()
                + Self::substep_width() * i as f64;
            let sample_point = Self::terrain_height_sample(x, seed);

            terrain_height_samples.push(sample_point);
        }

        let heightfield_collider = Collider::heightfield(
            terrain_height_samples.clone().into_iter().collect(),
            Vector::splat(Self::CHUNK_WIDTH as f64),
        );

        let mesh = Self::generate_terrain_mesh(&terrain_height_samples);

        (heightfield_collider, mesh)
    }

    pub fn generate_terrain_mesh(terrain_height_samples: &[f64]) -> Mesh {
        let mut verticies: Vec<[f32; 3]> = vec![];
        let mut indicies = vec![];
        let mut normals = vec![];

        let sample_heights = terrain_height_samples.iter().enumerate().peekable();
        for (i, height) in sample_heights {
            let height = *height as f32;

            verticies.push([
                (i as f32 * Self::substep_width() as f32) - Self::CHUNK_WIDTH / 2.0,
                -1000.0,
                0.0,
            ]); // Vertex 0
            verticies.push([
                (i as f32 * Self::substep_width() as f32) - Self::CHUNK_WIDTH / 2.0,
                (height * Self::CHUNK_WIDTH) - 1.0,
                0.0,
            ]);

            normals.push([0., 0., 1.]);
            normals.push([0., 0., 1.]);
        }

        for step in 0..Self::SUBSTEP_COUNT {
            let i = step * 2;
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
        info!("Vertex Count: {}", verticies.len());
        info!("Index Count: {}", indicies.len());

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verticies)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indicies))
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
