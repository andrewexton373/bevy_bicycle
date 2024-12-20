use bevy::{math::{DVec2, Vec2}, prelude::Event};

#[derive(Event)]
pub struct CameraZoomEvent(pub CameraZoomDirection);

pub enum CameraZoomDirection {
    In,
    Out
}

#[derive(Event)]
pub struct CameraPanEvent(pub CameraPanDirection);

pub enum CameraPanDirection{
    Up,
    Down,
    Left,
    Right
}

impl CameraPanDirection {
    pub fn as_dvec2(&self) -> Vec2 {
        match self {
            CameraPanDirection::Up => Vec2::Y,
            CameraPanDirection::Down => -Vec2::Y,
            CameraPanDirection::Left => -Vec2::X,
            CameraPanDirection::Right => Vec2::X,
        }
    }
}

#[derive(Event)]
pub struct CycleCameraModeEvent;