use std::{ops::Div, time::Instant};

use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, mut solution: &mut Solution, mut info: &mut Info) {
    let timer = Instant::now();
    let puzzle = get_puzzle(solver.index);

    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    solution.0 = 0;
    solution.1 = 0;

    // Part 1.
    for line in puzzle.lines() {
        let compartments = line.split_at(*&line.len().div(2));
        for char in compartments.0.chars() {
            if compartments.1.contains(char) {
                solution.0 += 1 + letters.find(char).unwrap();
                break;
            }
        }
    }

    // Part 2.
    for group in puzzle.lines().array_chunks::<3>() {
        for char in group[0].chars() {
            if group[1].contains(char) && group[2].contains(char) {
                solution.1 += 1 + letters.find(char).unwrap();
                break;
            }
        }
    }

    info.duration = timer.elapsed();
}

#[test]
fn test_03() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(3, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, 8039);
    assert_eq!(solution.1, 2510);
}
