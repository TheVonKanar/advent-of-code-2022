use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, mut solution: &mut Solution, _: &mut Info) {
    let puzzle = get_puzzle(solver.index);

    let mut loads: Vec<usize> = vec![];
    let mut current: usize = 0;

    for line in puzzle.lines() {
        match line.parse::<usize>() {
            Ok(calories) => current += calories,
            Err(_) => {
                loads.push(current);
                current = 0;
            }
        }
    }

    loads.sort_by(|a, b| b.cmp(a));

    solution.0 = loads[0].to_string();
    solution.1 = (loads[0] + loads[1] + loads[2]).to_string();
}

#[test]
fn test_01() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(1, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "69281");
    assert_eq!(solution.1, "201524");
}
