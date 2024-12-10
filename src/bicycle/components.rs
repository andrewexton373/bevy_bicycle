use avian2d::prelude::Collider;
use bevy::{math::{DVec2, Vec2}, prelude::Component, utils::HashMap};

use super::systems::AttachmentPoint;



#[derive(Component)]
pub struct Bicycle;

#[derive(Component)]
pub struct Frame;

#[derive(Component)]
pub struct BicycleFrame {
    pub rear_hub: Vec2,
    pub front_hub: Vec2,
    pub bottom_bracket: Vec2,
    pub seat_clamp: Vec2,
    pub stem_clamp: Vec2,
}

impl BicycleFrame {
    pub fn new() -> Self {
        BicycleFrame {
            rear_hub: Vec2::new(-40.0, 0.0),
            front_hub: Vec2::new(35.0, 0.0),
            bottom_bracket: Vec2::new(0.0, 0.0),
            seat_clamp: Vec2::new(-10.0, 20.0),
            stem_clamp: Vec2::new(30.0, 20.0),
        }
    }

    pub fn collider(&self) -> Collider {
        let frame_points_all: Vec<Vec2> =
        vec![self.rear_hub, self.bottom_bracket, self.seat_clamp, self.stem_clamp, self.front_hub];
        let frame_points_all_dvec2 = frame_points_all.iter().map(|v| v.as_dvec2()).collect();
        
        let frame_points_all_indicies: Vec<[u32; 2]> =
            vec![[0, 1], [1, 2], [2, 0], [2, 3], [1, 3], [3, 4]];

        let frame_collider =
            Collider::convex_decomposition(frame_points_all_dvec2, frame_points_all_indicies);

        frame_collider
    }

    pub fn attachment_points(&self) -> HashMap<AttachmentPoint, Vec2> {
        let mut attachment_points = HashMap::new();
        attachment_points.insert(AttachmentPoint::BottomBracket, self.bottom_bracket);
        attachment_points.insert(AttachmentPoint::FrontWheelFork, self.front_hub);
        attachment_points.insert(AttachmentPoint::RearWheelFork, self.rear_hub);
        
        attachment_points.clone()
    }
}

#[derive(Component)]
pub struct AttachmentPoints {
    bottom_bracket: DVec2,
    front_hub: DVec2,
    rear_hub: DVec2
}