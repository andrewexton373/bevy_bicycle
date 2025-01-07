use std::collections::HashMap;

use avian2d::prelude::{AngularVelocity, LinearVelocity, Rotation};
use bevy::{prelude::*, reflect::List};
use bevy_egui::{
    egui::{self, panel::TopBottomSide, Align2},
    EguiContexts,
};
use iyes_perf_ui::{entry::PerfUiEntry, prelude::PerfUiDefaultEntries};

use crate::{
    bicycle::{
        frame::BicycleFrame,
        groupset::{CassetteRadius, ChainringRadius, Cog},
        wheel::BicycleWheel,
    },
    camera::systems::CameraState,
    world::resources::{MaxTerrainChunkCount, TerrainSeed},
    BoundedQueue,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::plugin::UIPlugin;

#[derive(Default, Resource)]
pub struct UiState {
    chainring_radius: f32,
    cassette_radius: f32,
    max_terrain_chunk_count: u8,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BicycleStat {
    Speed,
    Grade,
    ChainringRPM,
    CassetteRPM,
    FrontWheelRPM,
    RearWheelRPM,
}

#[derive(Resource)]
pub struct BicycleStats {
    stats: HashMap<BicycleStat, BoundedQueue<f64>>,
}

const BICYCLE_STAT_SAMPLES: usize = 1000;

impl Default for BicycleStats {
    fn default() -> Self {
        let mut stats: HashMap<BicycleStat, BoundedQueue<f64>> = HashMap::new();

        for stat in BicycleStat::iter() {
            stats.insert(stat, BoundedQueue::new(BICYCLE_STAT_SAMPLES));
        }

        Self { stats }
    }
}

impl BicycleStats {
    pub fn get_avg(&self, stat: &BicycleStat) -> f64 {
        let stat_samples = self.stats.get(stat).unwrap().clone();
        let sum: f64 = stat_samples.clone().into_iter().sum();
        let count: f64 = stat_samples.len() as f64;
        sum / count
    }

    pub fn enqueue_value_for_stat(&mut self, stat: &BicycleStat, value: f64) {
        self.stats.entry(*stat).and_modify(|v| v.enqueue(value));
    }
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
            let _ = chainring_radius.replace_if_neq(ChainringRadius(ui_state.chainring_radius));
            let _ = cassette_radius.replace_if_neq(CassetteRadius(ui_state.cassette_radius));
            let _ = max_terrain_chunk_count
                .replace_if_neq(MaxTerrainChunkCount(ui_state.max_terrain_chunk_count));
        }
    }

    pub fn camera_window_ui(mut contexts: EguiContexts, camera_state: Res<State<CameraState>>) {
        egui::Window::new("Camera Information")
            .anchor(Align2::LEFT_TOP, bevy_egui::egui::Vec2::new(4.0, 28.0))
            .title_bar(false)
            .auto_sized()
            .show(contexts.ctx_mut(), |ui| {
                ui.label(format!("Camera Mode: {:?}", camera_state));
            });
    }

    pub fn top_panel_ui(
        mut ui_state: ResMut<UiState>,
        mut contexts: EguiContexts,
        terrain_seed: Res<TerrainSeed>,
        _camera_state: Res<State<CameraState>>,
    ) {
        egui::TopBottomPanel::new(TopBottomSide::Top, "Top Panel").show(contexts.ctx_mut(), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("Terrain Seed: {:?}", terrain_seed.0));
                ui.label(format!(
                    "FPS: {:?}",
                    PerfUiDefaultEntries::default().fps.label()
                ));
                ui.separator();

                ui.add(
                    egui::Slider::new(&mut ui_state.max_terrain_chunk_count, 4..=128).text("value"),
                );

                ui.label("Terrain Chunk Count:");
            });
        });
    }

    pub fn measure_bicycle_statistics(
        frame: Query<(&LinearVelocity, &Rotation), With<BicycleFrame>>,
        wheels: Query<(Entity, &BicycleWheel, &AngularVelocity)>,
        cogs: Query<(Entity, &Cog, &AngularVelocity)>,
        mut bicycle_stats: ResMut<BicycleStats>,
    ) {
        if wheels.is_empty() || cogs.is_empty() || frame.is_empty() {
            return;
        }

        let (lin_vel, rotation) = frame.single();
        bicycle_stats.enqueue_value_for_stat(&BicycleStat::Speed, lin_vel.length());
        let grade = 100.0 * rotation.sin / rotation.cos;
        bicycle_stats.enqueue_value_for_stat(&BicycleStat::Grade, grade);

        // Enqueue Wheel RPMs
        for (_wheel_ent, wheel, ang_vel) in wheels.iter() {
            let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);

            match wheel {
                BicycleWheel::Front => {
                    bicycle_stats.enqueue_value_for_stat(&BicycleStat::FrontWheelRPM, rpm)
                }
                BicycleWheel::Back => {
                    bicycle_stats.enqueue_value_for_stat(&BicycleStat::RearWheelRPM, rpm)
                }
            }
        }

        // Enqueue Cog RPMs
        for (_, cog, ang_vel) in cogs.iter() {
            let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);

            match cog {
                Cog::FrontChainring => {
                    bicycle_stats.enqueue_value_for_stat(&BicycleStat::ChainringRPM, rpm)
                }
                Cog::RearCassette => {
                    bicycle_stats.enqueue_value_for_stat(&BicycleStat::CassetteRPM, rpm)
                }
            }
        }
    }

    pub fn bottom_panel_ui(
        mut ui_state: ResMut<UiState>,
        mut contexts: EguiContexts,
        bicycle_stats: Res<BicycleStats>,
    ) {
        egui::TopBottomPanel::new(TopBottomSide::Bottom, "Bottom Panel").show(
            contexts.ctx_mut(),
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Bicycle Statistics");

                        ui.label(format!(
                            "Frame Grade: {:.1}",
                            bicycle_stats.get_avg(&BicycleStat::Grade)
                        ));

                        ui.label(format!(
                            "Bicycle Speed: {:.01}",
                            bicycle_stats.get_avg(&BicycleStat::Speed)
                        ));
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.heading("Wheel RPM");
                        ui.label(format!(
                            "{:?} RPM {:.0}",
                            BicycleWheel::Front,
                            bicycle_stats.get_avg(&BicycleStat::FrontWheelRPM)
                        ));
                        ui.label(format!(
                            "{:?} RPM {:.0}",
                            BicycleWheel::Back,
                            bicycle_stats.get_avg(&BicycleStat::RearWheelRPM)
                        ));
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.heading("COG RPM");

                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "COG {:?} RPM: {:.0}",
                                &Cog::FrontChainring,
                                bicycle_stats.get_avg(&BicycleStat::ChainringRPM)
                            ));
                            ui.label("Chainring Radius:");
                            ui.add(
                                egui::Slider::new(&mut ui_state.chainring_radius, 4.0..=15.0)
                                    .text("value"),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "COG {:?} RPM: {:.0}",
                                &Cog::RearCassette,
                                bicycle_stats.get_avg(&BicycleStat::CassetteRPM)
                            ));
                            ui.label("Cassette Radius:");
                            ui.add(
                                egui::Slider::new(&mut ui_state.cassette_radius, 4.0..=15.0)
                                    .text("value"),
                            );
                        });
                    });
                });
            },
        );
    }
}
