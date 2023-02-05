use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, solution: &mut Solution, _: &mut Info) {
    let puzzle = get_puzzle(solver.index);

    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in puzzle.lines() {
        let mut trees: Vec<u32> = Vec::new();
        for tree in line.chars().map(|c| c.to_digit(10).unwrap()) {
            trees.push(tree);
        }

        grid.push(trees);
    }

    let h = grid.len();
    let w = grid[0].len();
    let mut hidden = 0;
    let mut best_score = 0;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut vision = 4;
            let mut view = (0, 0, 0, 0);
            for i in (0..x).rev() {
                view.0 += 1;
                if grid[y][i] >= grid[y][x] {
                    vision -= 1;
                    break;
                }
            }

            for i in x + 1..w {
                view.1 += 1;
                if grid[y][i] >= grid[y][x] {
                    vision -= 1;
                    break;
                }
            }

            for i in (0..y).rev() {
                view.2 += 1;
                if grid[i][x] >= grid[y][x] {
                    vision -= 1;
                    break;
                }
            }

            for i in y + 1..h {
                view.3 += 1;
                if grid[i][x] >= grid[y][x] {
                    vision -= 1;
                    break;
                }
            }

            if vision == 0 {
                hidden += 1;
            }

            let score = view.0 * view.1 * view.2 * view.3;
            if score > best_score {
                best_score = score;
            }
        }
    }

    solution.0 = ((w * h) - hidden).to_string();
    solution.1 = best_score.to_string();
}

#[test]
fn test_08() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(8, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "1713");
    assert_eq!(solution.1, "268464");
}
