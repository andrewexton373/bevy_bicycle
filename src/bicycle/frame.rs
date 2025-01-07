use std::collections::BTreeMap;

use avian2d::prelude::*;
use bevy::ecs::system::{RunSystemOnce, SystemState};
use bevy::math::DVec2;
use bevy::prelude::*;

use crate::bicycle::groupset::spawn_groupset;
use crate::bicycle::wheel::{spawn_wheel, BicycleWheel};
use crate::camera::components::FollowCamera;
use crate::world::plugin::WorldTerrainPlugin;
use crate::world::resources::TerrainSeed;
use crate::GameLayer;

#[derive(Component)]
pub struct Frame;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FrameGeometry {
    RearHub = 1,
    FrontHub = 2,
    BottomBracket = 3,
    SeatClamp = 4,
    StemClamp = 5,
}

#[derive(Component)]
pub struct BicycleFrame {
    pub geometry: BTreeMap<FrameGeometry, DVec2>,
}

impl Default for BicycleFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl BicycleFrame {
    pub fn new() -> Self {
        BicycleFrame {
            geometry: vec![
                (FrameGeometry::RearHub, DVec2::new(-40.0, 0.0)),
                (FrameGeometry::BottomBracket, DVec2::new(0.0, 0.0)),
                (FrameGeometry::SeatClamp, DVec2::new(-10.0, 20.0)),
                (FrameGeometry::StemClamp, DVec2::new(30.0, 20.0)),
                (FrameGeometry::FrontHub, DVec2::new(35.0, 0.0)),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn collider(&self) -> Collider {
        let frame_points_all: Vec<DVec2> = self.geometry.iter().map(|item| *item.1).collect();

        let key_index =
            |key: &FrameGeometry| {
                self.geometry.keys().enumerate().find_map(|(i, k)| {
                    if k == key {
                        Some(i as u32)
                    } else {
                        None
                    }
                })
            };

        let rear_hub_to_bottom_bracket = [
            key_index(&FrameGeometry::RearHub).unwrap(),
            key_index(&FrameGeometry::BottomBracket).unwrap(),
        ];
        let bottom_bracket_to_seat_clamp = [
            key_index(&FrameGeometry::BottomBracket).unwrap(),
            key_index(&FrameGeometry::SeatClamp).unwrap(),
        ];
        let bottom_bracket_to_stem_clamp = [
            key_index(&FrameGeometry::BottomBracket).unwrap(),
            key_index(&FrameGeometry::StemClamp).unwrap(),
        ];
        let seat_clamp_to_rear_hub = [
            key_index(&FrameGeometry::SeatClamp).unwrap(),
            key_index(&FrameGeometry::RearHub).unwrap(),
        ];
        let seat_clamp_to_stem_clamp = [
            key_index(&FrameGeometry::SeatClamp).unwrap(),
            key_index(&FrameGeometry::StemClamp).unwrap(),
        ];
        let stem_clamp_to_front_hub = [
            key_index(&FrameGeometry::StemClamp).unwrap(),
            key_index(&FrameGeometry::FrontHub).unwrap(),
        ];

        let compound_segment_collider = Collider::compound(vec![
            (
                Position::default(),
                Rotation::default(),
                Collider::segment(
                    *self.geometry.get(&FrameGeometry::RearHub).unwrap(),
                    *self.geometry.get(&FrameGeometry::BottomBracket).unwrap(),
                ),
            ),
            // (
            //     Position::default(),
            //     Rotation::default(),
            //     Collider::segment(
            //         *self.geometry.get(&FrameGeometry::BottomBracket).unwrap(),
            //         *self.geometry.get(&FrameGeometry::SeatClamp).unwrap(),
            //     ),
            // ),
            (
                Position::default(),
                Rotation::default(),
                Collider::segment(
                    *self.geometry.get(&FrameGeometry::BottomBracket).unwrap(),
                    *self.geometry.get(&FrameGeometry::StemClamp).unwrap(),
                ),
            ),
            (
                Position::default(),
                Rotation::default(),
                Collider::segment(
                    *self.geometry.get(&FrameGeometry::SeatClamp).unwrap(),
                    *self.geometry.get(&FrameGeometry::RearHub).unwrap(),
                ),
            ),
            (
                Position::default(),
                Rotation::default(),
                Collider::segment(
                    *self.geometry.get(&FrameGeometry::SeatClamp).unwrap(),
                    *self.geometry.get(&FrameGeometry::StemClamp).unwrap(),
                ),
            ),
            (
                Position::default(),
                Rotation::default(),
                Collider::segment(
                    *self.geometry.get(&FrameGeometry::StemClamp).unwrap(),
                    *self.geometry.get(&FrameGeometry::FrontHub).unwrap(),
                ),
            ),
        ]);

        let frame_points_all_indicies: Vec<[u32; 2]> = vec![
            rear_hub_to_bottom_bracket,
            bottom_bracket_to_seat_clamp,
            seat_clamp_to_rear_hub,
            seat_clamp_to_stem_clamp,
            bottom_bracket_to_stem_clamp,
            stem_clamp_to_front_hub,
        ];

        // compound_segment_collider
        Collider::convex_decomposition(frame_points_all, frame_points_all_indicies);
        compound_segment_collider
    }
}

pub fn spawn_frame(world: &mut World) {
    let mut system_state: SystemState<(Res<TerrainSeed>, Query<&Transform, With<FollowCamera>>)> =
        SystemState::new(world);
    let (terrain_seed, camera_t) = system_state.get_mut(world);

    let bicycle_frame = BicycleFrame::new();
    let frame_collider = bicycle_frame.collider();

    let mut camera_pos = DVec2::ZERO;
    if let Ok(camera_t) = camera_t.get_single() {
        camera_pos = camera_t.translation.truncate().as_dvec2();
    }

    let spawn_height: f32 = 50.0
        + WorldTerrainPlugin::CHUNK_WIDTH
            * WorldTerrainPlugin::terrain_height_sample(camera_pos.x, terrain_seed.0) as f32;

    info!("SPAWN HEIGHT: {:?}", spawn_height);

    let _frame_id = world
        .spawn((
            BicycleFrame::new(),
            Name::new("Frame"),
            Transform::from_xyz(camera_pos.x as f32, spawn_height, 0.0),
            RigidBody::Dynamic,
            Mass(10.0),
            AngularInertia(0.1),
            CenterOfMass(bevy::prelude::Vec2::ZERO),
            Visibility::Inherited,
            frame_collider,
            CollisionMargin(0.5),
            CollisionLayers::new([GameLayer::Frame], [GameLayer::World]),
        ))
        .id();

    world
        .run_system_once_with(BicycleWheel::Front, spawn_wheel)
        .expect("Error Spawning Front Wheel");
    world
        .run_system_once_with(BicycleWheel::Back, spawn_wheel)
        .expect("Error Spawning Rear Wheel");
    world
        .run_system_once(spawn_groupset)
        .expect("Error Spawning Groupset");

    // TODO: causes lag due to physics interactions
    // world.run_system_once(ChainPlugin::spawn_chain);
}
