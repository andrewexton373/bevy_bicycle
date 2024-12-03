use avian2d::prelude::LinearVelocity;
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
// use bevy_parallax::{
//     Animation, CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent,
//     ParallaxMoveEvent, RepeatStrategy,
// };

use crate::bicycle::components::BicycleWheel;

use super::{components::FollowCamera, plugin::CameraPlugin};

impl CameraPlugin {
    pub fn setup_camera(
        mut commands: Commands,
        // mut create_parallax: EventWriter<CreateParallaxEvent>,
    ) {
        let camera = commands
            .spawn((
                FollowCamera,
                Camera2d,
                // Camera2dBundle {
                //     projection: OrthographicProjection {
                //         near: -1000.0,
                //         far: 1000.0,
                //         ..default()
                //     },
                //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
                //     ..default()
                // },
            ))
            // .insert(ParallaxCameraComponent::default())
            .id();

        // let event = CreateParallaxEvent {
        //     layers_data: vec![
        //         LayerData {
        //             speed: LayerSpeed::Bidirectional(0.99, 0.99),
        //             repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
        //             path: "media/mills-back.png".to_string(),
        //             tile_size: UVec2::new(1123, 794),
        //             cols: 6,
        //             rows: 1,
        //             scale: Vec2::splat(0.15),
        //             z: 0.6,
        //             position: Vec2::new(0., -20.),
        //             color: Color::BLACK,
        //             animation: Some(Animation::FPS(30.)),
        //             ..default()
        //         },
        //         LayerData {
        //             speed: LayerSpeed::Bidirectional(0.9, 0.9),
        //             repeat: LayerRepeat::horizontally(RepeatStrategy::MirrorBoth),
        //             path: "media/mills-back.png".to_string(),
        //             tile_size: UVec2::new(1123, 794),
        //             cols: 6,
        //             rows: 1,
        //             scale: Vec2::splat(0.8),
        //             position: Vec2::new(0., -50.),
        //             z: 0.9,
        //             color: Color::WHITE,
        //             index: 1,
        //             animation: Some(Animation::FPS(24.)),
        //             ..default()
        //         },
        //         LayerData {
        //             speed: LayerSpeed::Bidirectional(0.8, 0.8),
        //             repeat: LayerRepeat::horizontally(RepeatStrategy::MirrorBoth),
        //             path: "media/mills-front.png".to_string(),
        //             tile_size: UVec2::new(750, 434),
        //             cols: 6,
        //             rows: 1,
        //             z: 20.0,
        //             scale: Vec2::splat(1.5),
        //             position: Vec2::new(0., -100.),
        //             index: 3,
        //             animation: Some(Animation::FPS(20.)),
        //             ..default()
        //         },
        //     ],
        //     camera,
        // };
        // create_parallax.send(event);
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
        // mut move_event_writer: EventWriter<ParallaxMoveEvent>,
        time: Res<Time>,
    ) {
        // Follow the Front Circle
        for (circle, circle_t, circle_v) in player_query.iter() {
            if let BicycleWheel::Front = circle {
                let (camera, mut camera_t) = camera_query.single_mut();
                camera_t.translation = circle_t.translation;

                // move_event_writer.send(ParallaxMoveEvent {
                //     translation: Vec2::new(-circle_v.0.x as f32 * time.delta_seconds(), 0.0),
                //     camera,
                //     rotation: 0.,
                // });
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
