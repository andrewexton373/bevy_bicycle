use avian2d::prelude::LinearVelocity;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

use crate::bicycle::wheel::components::BicycleWheel;

use super::{components::FollowCamera, plugin::CameraPlugin};

impl CameraPlugin {
    pub fn setup_camera(
        mut commands: Commands,
    ) {
        let camera = commands
            .spawn((
                FollowCamera,
                Camera2d,
            ))
            .id();
    }

    pub fn camera_follow(
        player_query: Query<
            (&BicycleWheel, &Transform, &LinearVelocity),
            (With<BicycleWheel>, Without<FollowCamera>),
        >,
        mut camera_query: Query<
            (Entity, &mut Transform),
            (With<FollowCamera>, Without<BicycleWheel>),
        >,
        time: Res<Time>,
    ) {
        // Follow the Front Circle
        for (circle, circle_t, circle_v) in player_query.iter() {
            if let BicycleWheel::Back = circle {
                let (camera, mut camera_t) = camera_query.single_mut();
                camera_t.translation = circle_t.translation;
            }
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
