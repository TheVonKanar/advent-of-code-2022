use std::time::Instant;

use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, mut solution: &mut Solution, mut info: &mut Info) {
    let timer = Instant::now();
    let puzzle = get_puzzle(solver.index);

    let mut count_1 = 0;
    let mut count_2 = 0;

    for line in puzzle.lines() {
        let mut pairs: Vec<Vec<usize>> = Vec::new();
        for range in line.split(',') {
            pairs.push(range.split('-').map(|x| x.parse().unwrap()).collect());
        }

        // Part 1.
        if (pairs[0][0] >= pairs[1][0] && pairs[0][1] <= pairs[1][1])
            || (pairs[1][0] >= pairs[0][0] && pairs[1][1] <= pairs[0][1])
        {
            count_1 += 1;
        }

        // Part 2.
        if pairs[0][0] <= pairs[1][1] && pairs[1][0] <= pairs[0][1] {
            count_2 += 1;
        }
    }

    solution.0 = count_1.to_string();
    solution.1 = count_2.to_string();
    info.duration = timer.elapsed();
}

#[test]
fn test_04() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(4, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "453");
    assert_eq!(solution.1, "919");
}
