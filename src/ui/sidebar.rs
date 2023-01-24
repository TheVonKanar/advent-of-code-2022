use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_pkv::PkvStore;

use crate::{
    constants::PKV_SELECTED_SOLVER,
    solvers::{SelectSolverEvent, Solver},
};

pub fn draw_sidebar(
    mut egui_ctx: ResMut<EguiContext>,
    query: Query<&Solver>,
    mut ev_writer: EventWriter<SelectSolverEvent>,
    mut pkv: ResMut<PkvStore>,
) {
    egui::SidePanel::left("sidebar").show(egui_ctx.ctx_mut(), |ui| {
        for solver in query.iter() {
            ui.horizontal(|ui| {
                if ui.button(format!("Day #{:02}", solver.0)).clicked() {
                    ev_writer.send(SelectSolverEvent(solver.0));
                    pkv.set(PKV_SELECTED_SOLVER, &solver.0)
                        .expect("Failed to store Selected Day");
                }
            });
        }
    });
}
