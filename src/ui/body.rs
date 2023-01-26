use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_pkv::PkvStore;

use crate::constants::PKV_SELECTED_SOLVER;
use crate::solvers::{Solution, Solver};

pub fn draw_body(
    mut egui_ctx: ResMut<EguiContext>,
    query: Query<(&Solver, &Solution)>,
    pkv: Res<PkvStore>,
) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        if let Ok(selected_solver) = pkv.get::<usize>(PKV_SELECTED_SOLVER) {
            for (solver, solution) in query.iter() {
                if solver.0 == selected_solver {
                    ui.label(format!(
                        "Showing results for solver n°{}",
                        solver.0.to_string()
                    ));

                    ui.label(format!("Part 1: {}", solution.0.to_string()));
                    ui.label(format!("Part 2: {}", solution.1.to_string()));
                    ui.label(solution.2);
                }
            }
        }
    });
}
