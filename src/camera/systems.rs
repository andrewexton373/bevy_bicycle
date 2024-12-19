use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    math::VectorSpace,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridSettings};

use crate::bicycle::components::BicycleFrame;

use super::{components::FollowCamera, plugin::CameraPlugin};

impl CameraPlugin {
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
            Camera3dBundle {
                projection: Projection::Orthographic(OrthographicProjection::default_3d()),
                ..default()
            },
        ));
    }

    pub fn camera_follow(
        frame_query: Query<&Transform, (With<BicycleFrame>, Without<FollowCamera>)>,
        mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<BicycleFrame>)>,
    ) {
        // Follow the Bicycle Frame
        for frame_t in frame_query.iter() {
            let mut camera_t = camera_query.single_mut();
            camera_t.translation = frame_t.translation.truncate().extend(10.0);
        }
    }

    pub fn free_camera(
        mut camera: Query<&mut Transform, With<FollowCamera>>,
        keys: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
    ) {
        let mut movement_vector = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            movement_vector += Vec3::Y;
        }

        if keys.pressed(KeyCode::KeyA) {
            movement_vector -= Vec3::X;
        }

        if keys.pressed(KeyCode::KeyS) {
            movement_vector -= Vec3::Y;
        }

        if keys.pressed(KeyCode::KeyD) {
            movement_vector += Vec3::X;
        }

        if let Ok(mut camera_t) = camera.get_single_mut() {
            camera_t.translation += movement_vector * 10000.0 * time.delta_secs();
            // info!("MOVEMENT_VEC: {:?}", movement_vector);
            // info!("CAMERA_T: {:?}", camera_t.translation);
        }
    }

    pub fn zoom_scale(
        mut query_camera: Query<&mut Projection, With<FollowCamera>>,
        mut keyboard_input: EventReader<KeyboardInput>,
    ) {
        for event in keyboard_input.read() {
            if event.state == ButtonState::Pressed {
                // assume orthographic. do nothing if perspective.
                let Projection::Orthographic(ortho) = query_camera.single_mut().into_inner() else {
                    return;
                };

                match event.logical_key {
                    Key::ArrowUp => {
                        // zoom in
                        ortho.scale /= 1.25;
                    }
                    Key::ArrowDown => {
                        // zoom out
                        ortho.scale *= 1.25;
                    }
                    _ => {}
                }
            }
        }
    }
}
