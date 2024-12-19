use bevy::prelude::*;
use iyes_perf_ui::PerfUiPlugin;

use super::systems::UiState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_plugins(PerfUiPlugin)
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, UIPlugin::performance_ui)
            .add_systems(Update, UIPlugin::top_panel_ui)
            .add_systems(Update, UIPlugin::bottom_panel_ui)
            .add_systems(Update, UIPlugin::update_resources);
    }
}
