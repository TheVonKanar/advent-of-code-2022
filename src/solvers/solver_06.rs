use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, solution: &mut Solution, _: &mut Info) {
    let puzzle = get_puzzle(solver.index);
    let chars = puzzle.chars().collect::<Vec<char>>();

    fn get_marker_index(chars: &Vec<char>, size: usize) -> usize {
        let mut index = size - 1;
        'outer: for window in chars.windows(size) {
            index += 1;
            for char in window {
                if window.iter().filter(|c| *c == char).count() > 1 {
                    continue 'outer;
                }
            }

            break;
        }

        index
    }

    solution.0 = get_marker_index(&chars, 4).to_string();
    solution.1 = get_marker_index(&chars, 14).to_string();
}

#[test]
fn test_06() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(6, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "1779");
    assert_eq!(solution.1, "2635");
}
