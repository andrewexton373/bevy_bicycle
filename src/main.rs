use avian2d::{math::Vector, prelude::{AngularVelocity, Collider, Friction, Gravity, Joint, MassPropertiesBundle, PhysicsDebugPlugin, PhysicsSet, RevoluteJoint, RigidBody, SubstepCount, SweptCcd}, PhysicsPlugins};
use bevy::{color::palettes::css::{GRAY, RED}, input::{keyboard::{Key, KeyboardInput}, mouse::{MouseScrollUnit, MouseWheel}, ButtonState}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}};

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
    .insert_resource(SubstepCount(12))
    .add_systems(Startup, (setup_ground, setup_camera, setup_circle))
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
    let width: f32 = 10000.0;
    let height: f32 = 300.0;

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(width, height),
        Friction::new(1.0),
        SweptCcd::default(),
        ColorMesh2dBundle {
            mesh: meshes.add(Rectangle::new(width, height)).into(),
            material: materials.add(ColorMaterial::from_color(GRAY)),
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        }
    ));
}

#[derive(Component)]
enum PlayerCircle {
    Front,
    Back
}

#[derive(Component)]
struct Frame;

impl PlayerCircle {
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



fn setup_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut asset_server: Res<AssetServer>

) {
    let front_id = commands.spawn((
        PlayerCircle::Front,
        RigidBody::Dynamic,
        Collider::circle(PlayerCircle::size()),
        Friction::new(1.0),
        SweptCcd::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(PlayerCircle::size())).into(),
            
            material: custom_materials.add(CustomMaterial {
                color: LinearRgba::WHITE,
                color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        }
    )).id();

    let back_id = commands.spawn((
        PlayerCircle::Back,
        RigidBody::Dynamic,
        Collider::circle(PlayerCircle::size()),
        Friction::new(1.0),
        SweptCcd::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(PlayerCircle::size())).into(),
            
            material: custom_materials.add(CustomMaterial {
                color: LinearRgba::WHITE,
                color_texture: Some(asset_server.load("media/bike_spokes_2.png")),
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        }
    )).id();

    let frame_id = commands.spawn((
        Frame,
        RigidBody::Dynamic,
        Collider::segment(Vec2 { x: -40.0, y: 0.0 }, Vec2 { x: 40.0, y: 0.0 }),
        MassPropertiesBundle::new_computed(&Collider::rectangle(50.0, 50.0), 1.0),
    )).id();

    commands.spawn(
        RevoluteJoint::new(frame_id, front_id).with_local_anchor_1(Vec2 { x: -40.0, y: 0.0 }).with_angular_velocity_damping(0.0).with_linear_velocity_damping(0.0)
    );

    commands.spawn(
        RevoluteJoint::new(frame_id, back_id).with_local_anchor_1(Vec2 { x: 40.0, y: 0.0 }).with_angular_velocity_damping(0.0).with_linear_velocity_damping(0.0)
    );

    
}

fn spin_wheel(
    mut wheel_query: Query<(&PlayerCircle, &mut AngularVelocity), With<PlayerCircle>>,
    mut mouse_wheel_evt: EventReader<MouseWheel>
) {

    for &evt in mouse_wheel_evt.read() {
        match &evt.unit {
            MouseScrollUnit::Line => {

                for (wheel, mut ang_vel) in wheel_query.iter_mut() {
                    match wheel {
                        PlayerCircle::Back => {
                            ang_vel.0 += -10.0 * evt.y;
                            println!("HIT! {} ang vel: {}", evt.y, ang_vel.0);
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
    player_query: Query<(&PlayerCircle, &Transform), (With<PlayerCircle>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<PlayerCircle>)>
) {

    // Follow the Front Circle
    for (circle, circle_t) in player_query.iter() {
        match circle {
            PlayerCircle::Front => {
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
