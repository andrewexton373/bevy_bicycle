use bevy::prelude::*;
use iyes_perf_ui::PerfUiPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PerfUiPlugin)
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, UIPlugin::performance_ui)
            .add_systems(Update, UIPlugin::ui_system);
    }
}
