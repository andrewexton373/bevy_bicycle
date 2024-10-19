use avian2d::{math::{Vector, PI}, parry::{mass_properties::MassProperties, math::Rotation}, prelude::{AngularVelocity, Collider, CollisionMargin, ExternalTorque, FixedJoint, Friction, Gravity, Joint, LinearVelocity, Mass, MassPropertiesBundle, Physics, PhysicsDebugPlugin, PhysicsSet, Position, Restitution, RevoluteJoint, RigidBody, Sensor, SubstepCount, SweptCcd}, PhysicsPlugins};
use bevy::{color::palettes::{css::{BLUE, GRAY, RED}, tailwind::{BLUE_100, BLUE_400}}, input::{keyboard::{Key, KeyboardInput}, mouse::{MouseScrollUnit, MouseWheel}, ButtonState}, math::{dvec2, DVec2}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_parallax::{Animation, CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems, RepeatStrategy};

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
    ))
    .insert_resource(ClearColor(Color::from(BLUE_400)))
    .insert_resource(Gravity(Vector::NEG_Y * 100.0))
    // .insert_resource(Time::new_with(Physics::fixed_hz(144.0)))
    .insert_resource(SubstepCount(100))
    .add_systems(Startup, (setup_ground, setup_camera, setup_bicycle))
    .add_systems(Update, (zoom_scale, spin_wheel))
    .add_systems(Update, ui_system)
    .add_systems(PostUpdate,
camera_follow
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate)
            .before(ParallaxSystems)
    )
    .run();
}

fn ui_system(
    mut contexts: EguiContexts,
    rear_wheel_query: Query<(Entity, &BicycleWheel, &AngularVelocity)>
) {

    

    egui::Window::new("Bicyle Statistics").show(contexts.ctx_mut(), |ui| {

        for (wheel_ent, wheel, ang_vel) in rear_wheel_query.iter() {
            let rpm = -ang_vel.0 * 60.0 / (2.0 * PI);
            ui.label(format!("RPM {:?} {:.2}", wheel, rpm));
        }

        
    });
}

fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let width: f64 = 10000.0;
    let height: f64 = 300.0;

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(width, height),
        Friction::new(0.95),
        Restitution::new(0.0),
        SweptCcd::default(),
        ColorMesh2dBundle {
            mesh: meshes.add(Rectangle::new(width as f32, height as f32)).into(),
            material: materials.add(ColorMaterial::from_color(GRAY)),
            transform: Transform::from_xyz(0.0, -200.0, 10.0),
            ..default()
        }
    ));
}

#[derive(Component, Debug)]
enum BicycleWheel {
    Front,
    Back
}

#[derive(Component)]
struct Frame;

impl BicycleWheel {
    fn size() -> f32 {
        20.0
    }
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

fn setup_bicycle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut asset_server: Res<AssetServer>
) {
    let front_id = commands.spawn((
        BicycleWheel::Front,
        RigidBody::Dynamic,
        Collider::circle(BicycleWheel::size() as f64),
        CollisionMargin(1.0),
        Mass(1.0),
        Friction::new(0.95),
        Restitution::new(0.0),
        SweptCcd::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(BicycleWheel::size())).into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..default()
            },
            
            material: custom_materials.add(CustomMaterial {
                color: LinearRgba::WHITE,
                color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        }
    )).id();

    let back_id = commands.spawn((
        BicycleWheel::Back,
        RigidBody::Dynamic,
        Collider::circle(BicycleWheel::size() as f64),
        CollisionMargin(1.0),
        Mass(1.0),
        Friction::new(0.95),
        Restitution::new(0.0),
        SweptCcd::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(BicycleWheel::size())).into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..default()
            },
            material: custom_materials.add(CustomMaterial {
                color: LinearRgba::WHITE,
                color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        }
    )).id();

    let rear_hub = dvec2(-40.0, 0.0);
    let front_hub = dvec2(35.0, 0.0);
    let bottom_bracket = dvec2(0.0, 0.0);
    let seat_clamp = dvec2(-10.0, 20.0);
    let stem_clamp = dvec2(30.0, 20.0);

    let frame_points_all: Vec<DVec2> = vec![rear_hub, bottom_bracket, seat_clamp, stem_clamp, front_hub];
    let frame_points_all_indicies: Vec<[u32; 2]> = vec![[0, 1], [1, 2], [2, 0], [2, 3], [1, 3], [3, 4]];

    let frame_collider = Collider::convex_decomposition(frame_points_all, frame_points_all_indicies);

    let frame_id = commands.spawn((
        RigidBody::Dynamic,
        frame_collider,
        Sensor,

        MassPropertiesBundle {
            mass: Mass(10.0),
            ..default()
        }
    )).id();

    let crank_collider = Collider::polyline(vec![bottom_bracket + 8.0 * DVec2::Y, bottom_bracket + 8.0 * DVec2::NEG_Y], vec![[0, 1]].into());

    let crank = commands.spawn((
        RigidBody::Dynamic,
        crank_collider,
        Sensor,
        MassPropertiesBundle {
            mass: Mass(10.0),
            ..default()
        }
    )).id();

    commands.spawn(
        RevoluteJoint::new(frame_id, front_id).with_local_anchor_1(front_hub).with_compliance(0.0).with_angular_velocity_damping(0.0).with_linear_velocity_damping(0.0)
    );

    commands.spawn(
        RevoluteJoint::new(frame_id, back_id).with_local_anchor_1(rear_hub).with_compliance(0.0).with_angular_velocity_damping(0.0).with_linear_velocity_damping(0.0)
    );

    commands.spawn(
        RevoluteJoint::new(frame_id, crank).with_local_anchor_1(bottom_bracket).with_compliance(0.0).with_angular_velocity_damping(0.0).with_linear_velocity_damping(0.0)
    );

    
}

fn spin_wheel(
    mut wheel_query: Query<(&BicycleWheel, &mut ExternalTorque), With<BicycleWheel>>,
    mut mouse_wheel_evt: EventReader<MouseWheel>
) {

    for &evt in mouse_wheel_evt.read() {
        match &evt.unit {
            MouseScrollUnit::Line => {

                for (wheel, mut torque) in wheel_query.iter_mut() {
                    match wheel {
                        BicycleWheel::Back => {
                            *torque = ExternalTorque::new(-2000000.0 as f64 * evt.y as f64).with_persistence(true);
                            // ang_vel.0 += -10.0 as f64 * evt.y as f64;
                            println!("torque {}", torque.torque());
                        },
                        _ => {
            
                        }
                    }
                }

            },
            MouseScrollUnit::Pixel => {

            }
        }
    }

    
}

fn camera_follow(
    player_query: Query<(&BicycleWheel, &Transform, &LinearVelocity), (With<BicycleWheel>, Without<FollowCamera>)>,
    mut camera_query: Query<(Entity, &mut Transform), (With<FollowCamera>, Without<BicycleWheel>)>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    time: Res<Time>
) {

    // Follow the Front Circle
    for (circle, circle_t, circle_v) in player_query.iter() {
        match circle {
            BicycleWheel::Front => {
                let (camera, mut camera_t) = camera_query.single_mut();
                camera_t.translation = circle_t.translation;

                move_event_writer.send(ParallaxMoveEvent {
                    translation: Vec2::new(-circle_v.0.x as f32 * time.delta_seconds(), 0.0),
                    camera: camera,
                    rotation: 0.,
                });
            },
            _ => {
            }
        }
    }

}

#[derive(Component)]
struct FollowCamera;

fn setup_camera(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>
) {
    let camera = commands.spawn((
        FollowCamera,
        Camera2dBundle {
            projection: OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ))
    .insert(ParallaxCameraComponent::default())
    .id();

    let event = CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.99, 0.99),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "media/mills-back.png".to_string(),
                tile_size: UVec2::new(1123, 794),
                cols: 6,
                rows: 1,
                scale: Vec2::splat(0.15),
                z: 0.6,
                position: Vec2::new(0., -20.),
                color: Color::BLACK,
                animation: Some(Animation::FPS(30.)),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                repeat: LayerRepeat::horizontally(RepeatStrategy::MirrorBoth),
                path: "media/mills-back.png".to_string(),
                tile_size: UVec2::new(1123, 794),
                cols: 6,
                rows: 1,
                scale: Vec2::splat(0.8),
                position: Vec2::new(0., -50.),
                z: 0.9,
                color: Color::WHITE,
                index: 1,
                animation: Some(Animation::FPS(24.)),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.8, 0.8),
                repeat: LayerRepeat::horizontally(RepeatStrategy::MirrorBoth),
                path: "media/mills-front.png".to_string(),
                tile_size: UVec2::new(750, 434),
                cols: 6,
                rows: 1,
                z: 20.0,
                scale: Vec2::splat(1.5),
                position: Vec2::new(0., -100.),
                index: 3,
                animation: Some(Animation::FPS(20.)),
                ..default()
            },
        ],
        camera: camera,
    };
    create_parallax.send(event);

}

fn zoom_scale(
    mut query_camera: Query<&mut OrthographicProjection, With<FollowCamera>>,
    mut keyboard_input: EventReader<KeyboardInput>
) {

    for event in keyboard_input.read() {
        match event.state {
            ButtonState::Pressed => {

                let mut projection: Mut<'_, OrthographicProjection> = query_camera.single_mut();

                match event.logical_key {
                    Key::ArrowUp => {
                        projection.scale /= 1.25;
                    },
                    Key::ArrowDown => {
                        projection.scale *= 1.25;
                    },
                    _ => {

                    }
                }

            },
            _ => {

            }
        }
    }

}
