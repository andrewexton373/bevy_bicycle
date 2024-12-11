use bevy::prelude::{Component, Event};

use super::components::Cog;

#[derive(Event)]
pub struct SpawnGroupsetEvent;

#[derive(Event)]
pub struct SpawnAttachedEvent {
    pub cog: Cog,
}
