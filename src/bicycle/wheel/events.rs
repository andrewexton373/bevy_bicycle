use bevy::prelude::Event;

use super::components::BicycleWheel;

#[derive(Event)]
pub struct SpawnWheelEvent {
    pub wheel: BicycleWheel,
}
