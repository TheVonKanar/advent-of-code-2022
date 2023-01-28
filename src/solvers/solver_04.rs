use std::time::Instant;

use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, mut solution: &mut Solution, mut info: &mut Info) {
    let timer = Instant::now();
    let puzzle = get_puzzle(solver.index);

    solution.0 = 0;
    solution.1 = 0;

    for line in puzzle.lines() {
        let mut pairs: Vec<Vec<usize>> = Vec::new();
        for range in line.split(',') {
            pairs.push(range.split('-').map(|x| x.parse().unwrap()).collect());
        }

        // Part 1.
        if (pairs[0][0] >= pairs[1][0] && pairs[0][1] <= pairs[1][1])
            || (pairs[1][0] >= pairs[0][0] && pairs[1][1] <= pairs[0][1])
        {
            solution.0 += 1;
        }

        // Part 2.
        if pairs[0][0] <= pairs[1][1] && pairs[1][0] <= pairs[0][1] {
            solution.1 += 1;
        }
    }

    info.duration = timer.elapsed();
}

#[test]
fn test_04() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(4, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, 453);
    assert_eq!(solution.1, 919);
}
