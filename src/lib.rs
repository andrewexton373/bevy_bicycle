// These two generate a lot of false positives for Bevy systems
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
// This is not a library, so we don't need to worry about intra-doc links
#![allow(rustdoc::private_intra_doc_links)]

pub mod bicycle;
pub mod camera;
pub mod ui;
pub mod user_input;
pub mod world;

use avian2d::{
    math::Vector,
    prelude::{Gravity, PhysicsDebugPlugin, PhysicsLayer, SubstepCount},
    PhysicsPlugins,
};
use bevy::{
    color::palettes::{css::WHITE, tailwind::BLUE_400},
    pbr::wireframe::WireframeConfig,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
    utils::HashMap,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use bicycle::plugin::BicyclePlugin;
use camera::plugin::CameraPlugin;
use ui::plugin::UIPlugin;
use user_input::plugin::UserInputPlugin;
use world::plugin::WorldTerrainPlugin;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    World,
    Frame,
    Wheels,
    AttachmentPoints,
    Groupset,
    Chain,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let primary_window = Window {
            title: "Bevy Bicycle".to_string(),
            resolution: (1280.0, 720.0).into(),
            resizable: false,
            ..default()
        };

        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            EguiPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            // Material2dPlugin::<CustomMaterial>::default(),
            UIPlugin,
            WorldTerrainPlugin,
            CameraPlugin,
            BicyclePlugin,
            UserInputPlugin,
            Sprite3dPlugin,
            // WireframePlugin,
        ))
        .add_systems(Startup, load_png_assets)
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .init_state::<GameState>()
        .insert_resource(PNGAssets {
            assets: HashMap::new(),
        })
        // .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::from(BLUE_400)))
        .insert_resource(Gravity(Vector::NEG_Y * 100.0))
        .insert_resource(SubstepCount(120))
        // Wireframes can be configured with this resource. This can be changed at runtime.
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: WHITE.into(),
        });
    }
}

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    Ready,
}

#[derive(Resource)]
pub struct PNGAssets {
    pub assets: HashMap<String, Handle<Image>>,
}

fn load_png_assets(asset_server: Res<AssetServer>, mut png_assets: ResMut<PNGAssets>) {
    png_assets.assets.insert(
        "bicycle_wheel".to_string(),
        asset_server.load("media/bike_spokes_4.png"),
    );
}

fn setup(
    asset_server: Res<AssetServer>,
    png_assets: Res<PNGAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !png_assets.assets.iter().all(|(_, asset_handle)| {
        asset_server
            .get_load_state(asset_handle.id())
            .is_some_and(|s| s.is_loaded())
    }) {
        return;
    };
    // poll every frame to check if assets are loaded. Once they are, we can proceed with setup.
    info!("ASSETS LOADED -> READY");
    next_state.set(GameState::Ready);
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    // alpha_mode: AlphaMode,
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

use std::collections::VecDeque;

#[derive(Clone)]
struct BoundedQueue<T> {
    queue: VecDeque<T>,
    max_size: usize,
}

impl<T> BoundedQueue<T> {
    fn new(max_size: usize) -> Self {
        BoundedQueue {
            queue: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    fn enqueue(&mut self, item: T) {
        if self.queue.len() == self.max_size {
            self.queue.pop_front(); // Remove the oldest item
        }
        self.queue.push_back(item); // Add the new item
    }

    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front() // Remove the front item
    }

    fn peek(&self) -> Option<&T> {
        self.queue.front() // Peek at the front item
    }

    fn len(&self) -> usize {
        self.queue.len() // Get the current length of the queue
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty() // Check if the queue is empty
    }
}

// Implementing the Iterator trait for BoundedQueue
impl<T> Iterator for BoundedQueue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front() // Remove and return the front element
    }
}
