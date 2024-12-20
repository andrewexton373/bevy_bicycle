use bevy::prelude::Event;

#[derive(Event)]
pub struct CameraZoomEvent(pub CameraZoomDirection);

pub enum CameraZoomDirection {
    In,
    Out
}