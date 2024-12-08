use bevy::{math::DVec2, prelude::Component, utils::HashMap};

use super::systems::AttachmentPoint;

#[derive(Component, Debug, Clone, Copy)]
pub enum BicycleWheel {
    Front,
    Back,
}

impl BicycleWheel {
    pub fn size() -> f32 {
        20.0
    }
}

#[derive(Component)]
pub struct Frame;

#[derive(Component)]
pub struct Bicycle;
