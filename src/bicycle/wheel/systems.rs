use avian2d::prelude::*;
use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams};

use crate::{
    bicycle::{
        components::{BicycleFrame, FrameGeometry},
        systems::GameLayer,
    },
    PNGAssets,
};

use super::{components::BicycleWheel, events::SpawnWheelEvent, plugin::WheelPlugin};

impl WheelPlugin {
    pub fn spawn_wheel(
        trigger: Trigger<SpawnWheelEvent>,
        mut commands: Commands,
        frame: Query<(Entity, &Transform, &BicycleFrame)>,
        png_assets: Res<PNGAssets>,
        mut sprite_params: Sprite3dParams,
    ) {
        let evt = trigger.event();

        let (frame_ent, transform, frame) = frame.single();

        let mounting_point = match evt.wheel {
            BicycleWheel::Front => frame
                .geometry
                .get_key_value(&FrameGeometry::FrontHub)
                .unwrap(),
            BicycleWheel::Back => frame
                .geometry
                .get_key_value(&FrameGeometry::RearHub)
                .unwrap(),
        };

        let wheel = commands
            .spawn((
                evt.wheel,
                Name::new("Wheel"),
                RigidBody::Dynamic,
                Collider::circle(BicycleWheel::size() as f64),
                CollisionLayers::new([GameLayer::Wheels], [GameLayer::World]),
                DebugRender::default().with_collider_color(BLACK.into()),
                Mass(1.0),
                Friction::new(1.0),
                Restitution::new(0.0),
                Sprite3dBuilder {
                    image: png_assets.assets.get("bicycle_wheel").unwrap().clone(),
                    pixels_per_metre: 2.5,
                    alpha_mode: AlphaMode::Multiply,
                    unlit: true,
                    ..default()
                }
                .bundle(&mut sprite_params),
                Position::from(*mounting_point.1 + transform.translation.truncate().as_dvec2()),
            ))
            .id();

        commands.spawn((
            Name::new("Wheel Joint"),
            RevoluteJoint::new(frame_ent, wheel)
                .with_local_anchor_1(*mounting_point.1)
                .with_compliance(0.0001)
                .with_angular_velocity_damping(0.0)
                .with_linear_velocity_damping(1.00),
        ));
    }
}
