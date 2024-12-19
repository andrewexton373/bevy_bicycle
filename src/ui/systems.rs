use avian2d::prelude::{AngularVelocity, Rotation};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, TextureHandle},
    EguiContexts,
};
use iyes_perf_ui::prelude::PerfUiDefaultEntries;

use crate::{
    bicycle::{
        components::BicycleFrame,
        groupset::{
            components::Cog,
            resources::{CassetteRadius, ChainringRadius},
        },
        wheel::components::BicycleWheel,
    },
    world::resources::TerrainSeed,
};

use super::plugin::UIPlugin;

#[derive(Default, Resource)]
pub struct UiState {
    label: String,
    value: f32,
    inverted: bool,
    egui_texture_handle: Option<TextureHandle>,
    is_window_open: bool,
    chainring_radius: f32,
    cassette_radius: f32,
}

impl UIPlugin {
    pub fn performance_ui(mut commands: Commands) {
        commands.spawn(PerfUiDefaultEntries::default());
    }

    pub fn update_resources(
        ui_state: ResMut<UiState>,
        mut chainring_radius: ResMut<ChainringRadius>,
        mut cassette_radius: ResMut<CassetteRadius>,
    ) {
        if ui_state.is_changed() && !ui_state.is_added() {
            chainring_radius.replace_if_neq(ChainringRadius(ui_state.chainring_radius));
            cassette_radius.replace_if_neq(CassetteRadius(ui_state.cassette_radius));
        }
    }

    pub fn ui_system(
        mut ui_state: ResMut<UiState>,

        mut contexts: EguiContexts,
        frame: Query<&Rotation, With<BicycleFrame>>,
        rear_wheel_query: Query<(Entity, &BicycleWheel, &AngularVelocity)>,
        chainring_query: Query<(Entity, &Cog, &AngularVelocity)>,
        chainring_radius: ResMut<ChainringRadius>,
        terrain_seed: Res<TerrainSeed>,
    ) {
        egui::Window::new("Bicyle Simulator Information").show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Terrain Seed: {:?}", terrain_seed.0));

            let rotation = frame.single();

            ui.label(format!(
                "Frame Grade: {:.1}",
                100.0 * rotation.sin / rotation.cos
            ));

            for (wheel_ent, wheel, ang_vel) in rear_wheel_query.iter() {
                let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);
                ui.label(format!("RPM {:?} {:.2}", wheel, rpm));
            }

            for (_, _, ang_vel) in chainring_query.iter() {
                let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);
                ui.label(format!("COG RPM: {:.0}", rpm));
            }

            ui.label("Chainring Radius:");

            ui.add(egui::Slider::new(&mut ui_state.chainring_radius, 4.0..=15.0).text("value"));

            ui.label("Cassette Radius:");

            ui.add(egui::Slider::new(&mut ui_state.cassette_radius, 4.0..=15.0).text("value"));
        });
    }
}
