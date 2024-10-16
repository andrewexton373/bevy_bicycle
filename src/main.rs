use avian2d::{parry::{na::Dynamic, transformation::utils::transform}, prelude::{Collider, Collision, PhysicsDebugPlugin, PhysicsSet, RigidBody}, PhysicsPlugins};
use bevy::{color::palettes::css::RED, prelude::*, scene::ron::de};

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default()
    ))
    .insert_resource(ClearColor(Color::BLACK))
    .add_systems(Startup, (setup, setup_camera, setup_circle))
    .add_systems(Update, zoom_scale)
    .add_systems(PostUpdate,
camera_follow
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate)
    )
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(10000.0, 10.0),
        PbrBundle {
            mesh: meshes.add(Rectangle::new(10000.0, 10.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        }
    ));
}

#[derive(Component)]
struct PlayerCircle;

fn setup_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PlayerCircle,
        RigidBody::Dynamic,
        Collider::circle(100.0),
        PbrBundle {
            mesh: meshes.add(Circle::new(100.0)),
            material: materials.add(Color::WHITE),
            ..default()
        }
    ));
}

fn camera_follow(
    mut player_query: Query<&Transform, (With<PlayerCircle>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<PlayerCircle>)>
) {
    let player_t= player_query.single_mut();
    let mut camera_t = camera_query.single_mut();

    camera_t.translation = player_t.translation;
    println!("{}", camera_t.translation);
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
