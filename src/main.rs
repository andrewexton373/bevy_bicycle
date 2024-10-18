use avian2d::{math::Vector, parry::{mass_properties::MassProperties, math::Rotation}, prelude::{AngularVelocity, Collider, CollisionMargin, ExternalTorque, FixedJoint, Friction, Gravity, Joint, Mass, MassPropertiesBundle, Physics, PhysicsDebugPlugin, PhysicsSet, Position, Restitution, RevoluteJoint, RigidBody, Sensor, SubstepCount, SweptCcd}, PhysicsPlugins};
use bevy::{color::palettes::css::{GRAY, RED}, input::{keyboard::{Key, KeyboardInput}, mouse::{MouseScrollUnit, MouseWheel}, ButtonState}, math::{dvec2, DVec2}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}};

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
        Material2dPlugin::<CustomMaterial>::default(),
    ))
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(Gravity(Vector::NEG_Y * 100.0))
    // .insert_resource(Time::new_with(Physics::fixed_hz(144.0)))
    .insert_resource(SubstepCount(100))
    .add_systems(Startup, (setup_ground, setup_camera, setup_bicycle))
    .add_systems(Update, (zoom_scale, spin_wheel))
    .add_systems(PostUpdate,
camera_follow
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate)
    )
    .run();
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
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        }
    ));
}

#[derive(Component)]
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
        Friction::new(0.95),
        Restitution::new(0.0),
        SweptCcd::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(BicycleWheel::size())).into(),
            
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
        Friction::new(0.95),
        Restitution::new(0.0),
        SweptCcd::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(BicycleWheel::size())).into(),
            
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

    let mut frame_points_1: Vec<DVec2> = vec![];
    frame_points_1.push(rear_hub);
    frame_points_1.push(seat_clamp);
    frame_points_1.push(bottom_bracket);
    frame_points_1.push(rear_hub);

    let mut frame_points_2: Vec<DVec2> = vec![];
    frame_points_2.push(stem_clamp);
    frame_points_2.push(seat_clamp);
    frame_points_2.push(bottom_bracket);
    frame_points_2.push(stem_clamp);

    let mut fork_points: Vec<DVec2> = vec![];
    fork_points.push(stem_clamp);
    fork_points.push(front_hub);

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

    commands.spawn(
        RevoluteJoint::new(frame_id, front_id).with_local_anchor_1(front_hub).with_compliance(0.0).with_angular_velocity_damping(0.0).with_linear_velocity_damping(0.0)
    );

    commands.spawn(
        RevoluteJoint::new(frame_id, back_id).with_local_anchor_1(rear_hub).with_compliance(0.0).with_angular_velocity_damping(0.0).with_linear_velocity_damping(0.0)
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
    player_query: Query<(&BicycleWheel, &Transform), (With<BicycleWheel>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<BicycleWheel>)>
) {

    // Follow the Front Circle
    for (circle, circle_t) in player_query.iter() {
        match circle {
            BicycleWheel::Front => {
                let mut camera_t = camera_query.single_mut();
                camera_t.translation = circle_t.translation;
            },
            _ => {
            }
        }
    }

}

#[derive(Component)]
struct FollowCamera;

fn setup_camera(
    mut commands: Commands
) {
    commands.spawn((
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
    ));
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
