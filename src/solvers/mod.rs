mod solver_01;
mod solver_02;
mod solver_03;
mod solver_04;
mod solver_05;
mod solver_06;
mod solver_07;
mod solver_08;
mod solver_09;
mod solver_10;
mod solver_11;

use std::time::{Duration, Instant};

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
pub struct Solver {
    pub index: usize,
    pub resolve: fn(&Solver, &mut Solution, &mut Info),
}

impl Solver {
    pub fn new(index: usize, resolve: fn(&Solver, &mut Solution, &mut Info)) -> Solver {
        Solver { index, resolve }
    }
}

#[derive(Component)]
pub struct Solution(pub String, pub String);

impl Solution {
    pub fn new() -> Solution {
        Solution(String::new(), String::new())
    }
}

#[derive(Component)]
pub struct Info {
    pub duration: Duration,
    pub description: String,
}

impl Info {
    pub fn new() -> Info {
        Info {
            duration: Duration::ZERO,
            description: String::new(),
        }
    }
}

fn init_solvers(
    mut commands: Commands,
    pkv: Res<PkvStore>,
    mut ev_writer: EventWriter<SelectSolverEvent>,
) {
    commands.spawn(create_solver_bundle(1, solver_01::resolve));
    commands.spawn(create_solver_bundle(2, solver_02::resolve));
    commands.spawn(create_solver_bundle(3, solver_03::resolve));
    commands.spawn(create_solver_bundle(4, solver_04::resolve));
    commands.spawn(create_solver_bundle(5, solver_05::resolve));
    commands.spawn(create_solver_bundle(6, solver_06::resolve));
    commands.spawn(create_solver_bundle(7, solver_07::resolve));
    commands.spawn(create_solver_bundle(8, solver_08::resolve));
    commands.spawn(create_solver_bundle(9, solver_09::resolve));
    commands.spawn(create_solver_bundle(10, solver_10::resolve));
    commands.spawn(create_solver_bundle(11, solver_11::resolve));

    if let Ok(selected_solver) = pkv.get::<usize>(PKV_SELECTED_SOLVER) {
        ev_writer.send(SelectSolverEvent(selected_solver));
    }
}

fn run_solvers(
    mut query: Query<(&Solver, &mut Solution, &mut Info)>,
    mut ev_reader: EventReader<SelectSolverEvent>,
) {
    for ev in ev_reader.iter() {
        for (solver, mut solution, mut info) in query.iter_mut() {
            if ev.0 == solver.index {
                // Reset Solution and Info content.
                solution.0 = String::new();
                solution.1 = String::new();
                info.description = String::new();

                // Solve the problem.
                let timer = Instant::now();
                (solver.resolve)(solver, solution.as_mut(), info.as_mut());
                info.duration = timer.elapsed();
            }
        }
    }
}

pub(self) fn create_solver_bundle(
    index: usize,
    resolve: fn(&Solver, &mut Solution, &mut Info),
) -> (Solver, Solution, Info) {
    (Solver::new(index, resolve), Solution::new(), Info::new())
}

pub(self) fn get_puzzle(index: usize) -> String {
    std::fs::read_to_string(format!("puzzles/puzzle_{:02}.txt", index))
        .expect("Failed to load puzzle input")
}
