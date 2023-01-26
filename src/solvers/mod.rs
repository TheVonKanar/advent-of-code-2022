mod solver_01;
mod solver_02;

use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::constants::PKV_SELECTED_SOLVER;

pub struct SelectSolverEvent(pub usize);

pub struct SolversPlugin;

impl Plugin for SolversPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectSolverEvent>()
            .add_startup_system(init_solvers)
            .add_system(run_solvers);
    }
}

// Components.
#[derive(Component)]
pub struct Solver(pub usize, pub fn(String) -> (usize, usize, &'static str));

#[derive(Component)]
pub struct Solution(pub usize, pub usize, pub &'static str);

fn init_solvers(
    mut commands: Commands,
    pkv: Res<PkvStore>,
    mut ev_writer: EventWriter<SelectSolverEvent>,
) {
    commands.spawn((Solver(1, solver_01::resolve), Solution(0, 0, "")));
    commands.spawn((Solver(2, solver_02::resolve), Solution(0, 0, "")));

    if let Ok(selected_solver) = pkv.get::<usize>(PKV_SELECTED_SOLVER) {
        ev_writer.send(SelectSolverEvent(selected_solver));
    }
}

fn run_solvers(
    mut query: Query<(&Solver, &mut Solution)>,
    mut ev_reader: EventReader<SelectSolverEvent>,
) {
    for ev in ev_reader.iter() {
        for (solver, mut solution) in query.iter_mut() {
            if ev.0 == solver.0 {
                let resolution = solver.1(get_puzzle(solver.0));
                solution.0 = resolution.0;
                solution.1 = resolution.1;
                solution.2 = resolution.2;
            }
        }
    }
}

pub(self) fn get_puzzle(index: usize) -> String {
    std::fs::read_to_string(format!("puzzles/puzzle_{:02}.txt", index))
        .expect("Failed to load puzzle input")
}
