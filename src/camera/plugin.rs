use avian2d::prelude::PhysicsSet;
use bevy::prelude::*;
use bevy_infinite_grid::InfiniteGridPlugin;
// use bevy_parallax::ParallaxSystems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InfiniteGridPlugin)
            .add_systems(Startup, CameraPlugin::setup_camera)
            .add_systems(Startup, CameraPlugin::setup_infinite_grid)

            .add_systems(Update, CameraPlugin::zoom_scale)
            .add_systems(
                PostUpdate,
                (
                    CameraPlugin::camera_follow
                            .after(PhysicsSet::Sync)
                            .before(TransformSystem::TransformPropagate),
                    // CameraPlugin::free_camera
                    //     .after(PhysicsSet::Sync)
                    //     .before(TransformSystem::TransformPropagate),
                ),
            );
    }
}
