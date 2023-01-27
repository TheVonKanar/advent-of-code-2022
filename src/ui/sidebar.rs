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
                if ui.button(format!("Day #{:02}", solver.index)).clicked() {
                    ev_writer.send(SelectSolverEvent(solver.index));
                    pkv.set(PKV_SELECTED_SOLVER, &solver.index)
                        .expect("Failed to store Selected Day");
                }
            });
        }
    });
}
