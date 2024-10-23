pub mod ui;
pub mod world;
pub mod camera;
pub mod bicycle;

use std::{f32::consts::PI};

use avian2d::{
    math::Vector, parry::na::{Point2, Rotation2}, prelude::{
        AngularVelocity, Collider, CollisionMargin, ExternalTorque, Friction, Gravity, Joint,
        LinearVelocity, Mass, MassPropertiesBundle, PhysicsDebugPlugin, PhysicsSet, Restitution,
        RevoluteJoint, RigidBody, Sensor, SubstepCount, SweptCcd,
    }, PhysicsPlugins
};
use bevy::{
    color::palettes::{css::GRAY, tailwind::BLUE_400},
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{MouseScrollUnit, MouseWheel},
        ButtonState,
    },
    math::{dvec2, vec3, DVec2},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_parallax::{
    Animation, CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent,
    ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems, RepeatStrategy,
};
use bicycle::plugin::BicyclePlugin;
use camera::plugin::CameraPlugin;
use ui::plugin::UIPlugin;
use world::plugin::WorldPlugin;

fn main() {
    let primary_window = Window {
        title: "Bevy Bicycle".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            EguiPlugin,
            ParallaxPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            Material2dPlugin::<CustomMaterial>::default(),
            UIPlugin,
            WorldPlugin,
            CameraPlugin,
            BicyclePlugin
        ))
        .insert_resource(ClearColor(Color::from(BLUE_400)))
        .insert_resource(Gravity(Vector::NEG_Y * 100.0))
        // .insert_resource(Time::new_with(Physics::fixed_hz(144.0)))
        .insert_resource(SubstepCount(10))

        .run();
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_2d.wgsl".into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}
