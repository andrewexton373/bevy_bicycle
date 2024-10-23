use bevy::prelude::Component;

#[derive(Component, Debug)]
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

