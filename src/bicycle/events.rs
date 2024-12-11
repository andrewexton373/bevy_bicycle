use bevy::prelude::Event;

#[derive(Event)]
pub struct SpawnBicycleEvent;

#[derive(Event)]
pub struct SpawnFrameEvent;

#[derive(Event)]
pub struct SpawnCrankEvent;

#[derive(Event)]
pub struct SpawnAttachmentPointEvent;
