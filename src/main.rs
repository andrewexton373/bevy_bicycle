use avian2d::{math::Vector, parry::{na::Dynamic, transformation::utils::transform}, prelude::{AngularVelocity, Collider, Collision, ExternalAngularImpulse, ExternalTorque, Friction, Gravity, Joint, MassPropertiesBundle, PhysicsDebugPlugin, PhysicsSet, RevoluteJoint, RigidBody, SubstepCount, SweptCcd}, PhysicsPlugins};
use bevy::{color::palettes::css::RED, input::mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel}, prelude::*, scene::ron::de};

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default()
    ))
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(Gravity(Vector::NEG_Y * 100.0))
    .insert_resource(SubstepCount(50))
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
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(10000.0, 10.0),
        Friction::new(1.0),
        SweptCcd::default(),
        PbrBundle {
            mesh: meshes.add(Rectangle::new(10000.0, 10.0)),
            material: materials.add(Color::WHITE),
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

fn setup_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let front_id = commands.spawn((
        PlayerCircle::Front,
        RigidBody::Dynamic,
        Collider::circle(PlayerCircle::size()),
        Friction::new(1.0),
        SweptCcd::default(),
        PbrBundle {
            mesh: meshes.add(Circle::new(PlayerCircle::size())),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(40.0, 0.0, 0.0),
            ..default()
        }
    )).id();

    let back_id = commands.spawn((
        PlayerCircle::Back,
        RigidBody::Dynamic,
        Collider::circle(PlayerCircle::size()),
        Friction::new(1.0),
        SweptCcd::default(),
        PbrBundle {
            mesh: meshes.add(Circle::new(PlayerCircle::size())),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(-40.0, 0.0, 0.0),
            ..default()
        }
    )).id();

    let spoke = commands.spawn((
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "X",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_background_color(Color::from(RED)),

        // Collider::segment(Vec2 { x: -10.0, y: 0.0 }, Vec2 { x: 10.0, y: 0.0 })
    )).id();

    commands.entity(back_id).add_child(spoke);


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
                            ang_vel.0 += -100000.0 * evt.y;
                            println!("HIT! {}", evt.y);
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
) {
    let mut projection = query_camera.single_mut();
    // zoom in
    // projection.scale /= 1.25;
    // zoom out
    // projection.scale *= 1.25;
}
