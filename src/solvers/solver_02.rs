use std::time::Instant;

use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, mut solution: &mut Solution, mut info: &mut Info) {
    let timer = Instant::now();
    let puzzle = get_puzzle(solver.index);

    let mut score_1 = 0;
    let mut score_2 = 0;

    for line in puzzle.lines() {
        let cols: Vec<&str> = line.split(" ").collect();

        // Part 1: XYZ is Rock/Paper/Scissors.
        score_1 += match cols[0] {
            "A" => match cols[1] {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3 + 0,
                &_ => 0,
            },
            "B" => match cols[1] {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                &_ => 0,
            },
            "C" => match cols[1] {
                "X" => 1 + 6,
                "Y" => 2 + 0,
                "Z" => 3 + 3,
                &_ => 0,
            },
            &_ => 0,
        };

        // Part 2: XYZ is Lose/Draw/Win.
        score_2 += match cols[0] {
            "A" => match cols[1] {
                "X" => 0 + 3,
                "Y" => 3 + 1,
                "Z" => 6 + 2,
                &_ => 0,
            },
            "B" => match cols[1] {
                "X" => 0 + 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                &_ => 0,
            },
            "C" => match cols[1] {
                "X" => 0 + 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                &_ => 0,
            },
            &_ => 0,
        }
    }

    solution.0 = score_1.to_string();
    solution.1 = score_2.to_string();
    info.duration = timer.elapsed();
}

#[test]
fn test_02() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(2, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "11906");
    assert_eq!(solution.1, "11186");
}
