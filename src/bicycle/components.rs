use std::collections::BTreeMap; // itertools = "0.8"

use avian2d::prelude::Collider;
use bevy::{
    math::Vec2,
    prelude::Component,
};

// use super::systems::AttachmentPoint;

#[derive(Component)]
pub struct Bicycle;

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
    pub gemometry: BTreeMap<FrameGeometry, Vec2>,
}

impl Default for BicycleFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl BicycleFrame {
    pub fn new() -> Self {
        BicycleFrame {
            gemometry: vec![
                (FrameGeometry::RearHub, Vec2::new(-40.0, 0.0)),
                (FrameGeometry::BottomBracket, Vec2::new(0.0, 0.0)),
                (FrameGeometry::SeatClamp, Vec2::new(-10.0, 20.0)),
                (FrameGeometry::StemClamp, Vec2::new(30.0, 20.0)),
                (FrameGeometry::FrontHub, Vec2::new(35.0, 0.0)),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn collider(&self) -> Collider {
        let frame_points_all: Vec<Vec2> = self.gemometry.iter().map(|item| *item.1).collect();
        let frame_points_all_dvec2 = frame_points_all.iter().map(|v| v.as_dvec2()).collect();

        let key_index =
            |key: &FrameGeometry| {
                self.gemometry.keys().enumerate().find_map(|(i, k)| {
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

        let frame_points_all_indicies: Vec<[u32; 2]> = vec![
            rear_hub_to_bottom_bracket,
            bottom_bracket_to_seat_clamp,
            seat_clamp_to_rear_hub,
            seat_clamp_to_stem_clamp,
            bottom_bracket_to_stem_clamp,
            stem_clamp_to_front_hub,
        ];

        

        Collider::convex_decomposition(frame_points_all_dvec2, frame_points_all_indicies)
    }
}
