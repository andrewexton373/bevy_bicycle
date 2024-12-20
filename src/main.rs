pub mod bicycle;
pub mod camera;
pub mod user_input;
pub mod ui;
pub mod world;

use std::iter::Map;

use avian2d::{
    math::Vector,
    prelude::{Gravity, PhysicsDebugPlugin, SubstepCount},
    PhysicsPlugins,
};
use bevy::{
    color::palettes::{css::WHITE, tailwind::BLUE_400}, input::InputPlugin, pbr::wireframe::{WireframeConfig, WireframePlugin}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin}, utils::HashMap
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::{Sprite3dParams, Sprite3dPlugin};
use bicycle::plugin::BicyclePlugin;
use camera::plugin::CameraPlugin;
use ui::plugin::UIPlugin;
use user_input::plugin::UserInputPlugin;
    use world::plugin::WorldTerrainPlugin;
    use itertools::Itertools;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameState { #[default] Loading, Ready }


#[derive(Resource)]
pub struct PNGAssets {
    pub assets: HashMap<String, Handle<Image>>
}

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
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            // Material2dPlugin::<CustomMaterial>::default(),
            UIPlugin,
            WorldTerrainPlugin,
            CameraPlugin,
            BicyclePlugin,
            UserInputPlugin,
            Sprite3dPlugin,
            WireframePlugin,
        ))
        .add_systems(Startup, load_png_assets)
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .init_state::<GameState>()
        .insert_resource(PNGAssets {assets: HashMap::new()})
        //.add_plugins(WorldInspectorPlugin::new())
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
        })
        .run();
}

fn load_png_assets(
    asset_server: Res<AssetServer>,
    mut png_assets: ResMut<PNGAssets>
) {
    png_assets.assets.insert("bicycle_wheel".to_string(), asset_server.load("media/bike_spokes_4.png"));
}

fn setup(
    asset_server: Res<AssetServer>,
    png_assets: Res<PNGAssets>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut sprite_params: Sprite3dParams
) {
    if !png_assets.assets.iter().all(|(_, asset_handle)| asset_server.get_load_state(asset_handle.id()).is_some_and(|s| s.is_loaded())) { return };
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
