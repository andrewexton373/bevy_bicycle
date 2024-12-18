use avian2d::prelude::LinearVelocity;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    }, math::VectorSpace, prelude::*
};

use crate::bicycle::{components::BicycleFrame, wheel::components::BicycleWheel};

use super::{components::FollowCamera, plugin::CameraPlugin};

impl CameraPlugin {
    pub fn setup_camera(
        mut commands: Commands,
    ) {
        commands
            .spawn((
                FollowCamera,
                Camera2d,
            ));
    }

    pub fn camera_follow(
        frame_query: Query<
            (&Transform),
            (With<BicycleFrame>, Without<FollowCamera>),
        >,
        mut camera_query: Query<
            (&mut Transform),
            (With<FollowCamera>, Without<BicycleFrame>),
        >,
    ) {
        // Follow the Bicycle Frame
        for frame_t in frame_query.iter() {
            let mut camera_t = camera_query.single_mut();
            camera_t.translation = frame_t.translation;
        }
    }

    pub fn free_camera(
        mut camera: Query<&mut Transform, With<FollowCamera>>,
        keys: Res<ButtonInput<KeyCode>>,
        time: Res<Time>
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
            info!("MOVEMENT_VEC: {:?}", movement_vector);

            info!("CAMERA_T: {:?}", camera_t.translation);
        }

    }

    pub fn zoom_scale(
        mut query_camera: Query<&mut OrthographicProjection, With<FollowCamera>>,
        mut keyboard_input: EventReader<KeyboardInput>,
    ) {
        for event in keyboard_input.read() {
            if event.state == ButtonState::Pressed {
                let mut projection: Mut<'_, OrthographicProjection> = query_camera.single_mut();

                match event.logical_key {
                    Key::ArrowUp => {
                        projection.scale /= 1.25;
                    }
                    Key::ArrowDown => {
                        projection.scale *= 1.25;
                    }
                    _ => {}
                }
            }
        }
    }
}
