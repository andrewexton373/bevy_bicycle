use bevy::prelude::Component;

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