use avian2d::prelude::AngularVelocity;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::bicycle::{groupset::components::Cog, wheel::components::BicycleWheel};

use super::plugin::UIPlugin;

impl UIPlugin {
    pub fn ui_system(
        mut contexts: EguiContexts,
        rear_wheel_query: Query<(Entity, &BicycleWheel, &AngularVelocity)>,
        chainring_query: Query<(Entity, &Cog, &AngularVelocity)>,

    ) {
        egui::Window::new("Bicyle Statistics").show(contexts.ctx_mut(), |ui| {
            for (wheel_ent, wheel, ang_vel) in rear_wheel_query.iter() {
                let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);
                ui.label(format!("RPM {:?} {:.2}", wheel, rpm));
            }

            for (_, _, ang_vel) in chainring_query.iter() {
                let rpm = -ang_vel.0 * 60.0 / (2.0 * std::f64::consts::PI);
                ui.label(format!("COG RPM: {:.0}", rpm));
            }
        });
    }
}
