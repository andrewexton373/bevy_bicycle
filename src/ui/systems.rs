use avian2d::prelude::{AngularVelocity, LinearVelocity, Rotation};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, panel::TopBottomSide, TextureHandle},
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
    world::resources::{MaxTerrainChunkCount, TerrainSeed},
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
    max_terrain_chunk_count: u8,
}

impl UIPlugin {
    pub fn performance_ui(mut commands: Commands) {
        commands.spawn(PerfUiDefaultEntries::default());
    }

    pub fn update_resources(
        ui_state: ResMut<UiState>,
        mut chainring_radius: ResMut<ChainringRadius>,
        mut cassette_radius: ResMut<CassetteRadius>,
        mut max_terrain_chunk_count: ResMut<MaxTerrainChunkCount>,
    ) {
        if ui_state.is_changed() && !ui_state.is_added() {
            chainring_radius.replace_if_neq(ChainringRadius(ui_state.chainring_radius));
            cassette_radius.replace_if_neq(CassetteRadius(ui_state.cassette_radius));
            max_terrain_chunk_count
                .replace_if_neq(MaxTerrainChunkCount(ui_state.max_terrain_chunk_count));
        }
    }

    pub fn top_panel_ui(
        mut ui_state: ResMut<UiState>,
        mut contexts: EguiContexts,
        terrain_seed: Res<TerrainSeed>,
    ) {
        egui::TopBottomPanel::new(TopBottomSide::Top, "Top Panel").show(contexts.ctx_mut(), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("Terrain Seed: {:?}", terrain_seed.0));
                ui.separator();

                ui.label("Terrain Chunk Count:");

                ui.add(
                    egui::Slider::new(&mut ui_state.max_terrain_chunk_count, 4..=128).text("value"),
                );
            });
        });
    }

    pub fn bottom_panel_ui(
        mut ui_state: ResMut<UiState>,
        mut contexts: EguiContexts,
        frame: Query<(&LinearVelocity, &Rotation), With<BicycleFrame>>,
        rear_wheel_query: Query<(Entity, &BicycleWheel, &AngularVelocity)>,
        chainring_query: Query<(Entity, &Cog, &AngularVelocity)>,
    ) {
        egui::TopBottomPanel::new(TopBottomSide::Bottom, "Bottom Panel").show(
            contexts.ctx_mut(),
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Bicycle Statistics");

                        let (lin_vel, rotation) = frame.single();

                        ui.label(format!(
                            "Frame Grade: {:.1}",
                            100.0 * rotation.sin / rotation.cos
                        ));

                        ui.label(format!("Bicycle Speed: {:.01}", lin_vel.length().abs()));
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.heading("Wheel RPM");
                        for (wheel_ent, wheel, ang_vel) in rear_wheel_query.iter() {
                            let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);
                            ui.label(format!("{:?} RPM {:.0}", wheel, rpm));
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.heading("COG RPM");
                        for (_, cog, ang_vel) in chainring_query.iter() {
                            ui.horizontal(|ui| match cog {
                                Cog::FrontChainring => {
                                    let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);
                                    ui.label(format!("COG {:?} RPM: {:.0}", cog, rpm));

                                    ui.label("Chainring Radius:");

                                    ui.add(
                                        egui::Slider::new(
                                            &mut ui_state.chainring_radius,
                                            4.0..=15.0,
                                        )
                                        .text("value"),
                                    );
                                }
                                Cog::RearCassette => {
                                    let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);
                                    ui.label(format!("COG {:?} RPM: {:.0}", cog, rpm));
                                    ui.label("Cassette Radius:");

                                    ui.add(
                                        egui::Slider::new(
                                            &mut ui_state.cassette_radius,
                                            4.0..=15.0,
                                        )
                                        .text("value"),
                                    );
                                }
                            });
                        }
                    });
                });
            },
        );
    }
}
