use avian2d::prelude::PhysicsSet;
use bevy::prelude::*;
// use bevy_parallax::ParallaxSystems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, CameraPlugin::setup_camera)
            .add_systems(Update, CameraPlugin::zoom_scale)
            .add_systems(
                PostUpdate,
                (
                    CameraPlugin::camera_follow
                        .after(PhysicsSet::Sync)
                        .before(TransformSystem::TransformPropagate)
                    // .before(ParallaxSystems),
                ),
            );
    }
}
