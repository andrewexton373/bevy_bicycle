use bevy::prelude::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};

use crate::bicycle::components::BicycleFrame;

use super::{
    components::FollowCamera,
    events::{CameraPanEvent, CameraZoomDirection, CameraZoomEvent, CycleCameraModeEvent},
    plugin::CameraPlugin,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum CameraState {
    #[default]
    Follow,
    Free,
}

impl CameraPlugin {
    pub fn handle_cycle_camera_mode_event(
        mut events: EventReader<CycleCameraModeEvent>,
        state: Res<State<CameraState>>,
        mut next_state: ResMut<NextState<CameraState>>,
    ) {
        for _evt in events.read() {
            match state.get() {
                CameraState::Follow => next_state.set(CameraState::Free),
                CameraState::Free => next_state.set(CameraState::Follow),
            }
        }
    }

    pub fn setup_infinite_grid(mut commands: Commands) {
        commands.spawn((
            InfiniteGridBundle {
                settings: InfiniteGridSettings {
                    fadeout_distance: 1000000.0,
                    ..default()
                },
                ..default()
            },
            Name::new("Infinite Grid"),
        ));
    }

    pub fn setup_camera(mut commands: Commands) {
        commands.spawn((
            FollowCamera,
            Camera3d::default(),
            Projection::Orthographic(OrthographicProjection::default_3d()),
        ));
    }

    pub fn camera_follow(
        frame_query: Query<&Transform, (With<BicycleFrame>, Without<FollowCamera>)>,
        mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<BicycleFrame>)>,
    ) {
        // Follow the Bicycle Frame
        for frame_t in frame_query.iter() {
            let mut camera_t = camera_query.single_mut();
            camera_t.translation = frame_t.translation.truncate().extend(100.0);
        }
    }

    pub fn free_camera(
        mut camera: Query<&mut Transform, With<FollowCamera>>,
        mut pan_events: EventReader<CameraPanEvent>,
        time: Res<Time>,
    ) {
        for evt in pan_events.read() {
            let movement_vector = evt.0.as_dvec2();

            if let Ok(mut camera_t) = camera.get_single_mut() {
                camera_t.translation += (movement_vector * 1000.0 * time.delta_secs()).extend(0.0);
            }
        }
    }

    pub fn handle_zoom_event(
        mut events: EventReader<CameraZoomEvent>,
        mut query_camera: Query<&mut Projection, With<FollowCamera>>,
    ) {
        // assume orthographic. do nothing if perspective.
        let Projection::Orthographic(ortho) = query_camera.single_mut().into_inner() else {
            return;
        };

        for evt in events.read() {
            match evt.0 {
                CameraZoomDirection::In => {
                    ortho.scale /= 1.02;
                }
                CameraZoomDirection::Out => {
                    ortho.scale *= 1.02;
                }
            }
        }
    }
}
