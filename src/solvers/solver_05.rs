use std::cmp;

use regex::Regex;

use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, solution: &mut Solution, _: &mut Info) {
    let puzzle = get_puzzle(solver.index);

    let mut moving = false;
    let mut crates_1: Vec<Vec<char>> = Vec::new();
    let mut crates_2: Vec<Vec<char>> = Vec::new();
    for line in puzzle.lines() {
        if moving {
            if line.is_empty() {
                continue;
            }

            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            if let Some(caps) = re.captures(line) {
                let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
                let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
                let move_count = cmp::min(
                    caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    crates_1[from].len(),
                );

                // Part 1: Each crate one by one.
                for _ in 0..move_count {
                    if let Some(moving_crate) = crates_1[from].pop() {
                        crates_1[to].push(moving_crate);
                    }
                }

                // Part 2: All crates at once (preserving order).
                let stack_len = crates_2[from].len();
                let moving_crates = crates_2[from]
                    .drain(stack_len.saturating_sub(move_count)..stack_len)
                    .collect::<Vec<char>>();
                for moving_crate in moving_crates {
                    crates_2[to].push(moving_crate);
                }
            }
        } else {
            let chars: Vec<char> = line.chars().collect();
            if chars[1] == '1' {
                moving = true;
                for stack in crates_1.iter_mut() {
                    stack.reverse();
                }

                crates_2 = crates_1.clone();
                continue;
            }

            let mut char_index = 1;
            let mut stack_index = 0;

            while char_index < chars.len() {
                if stack_index >= crates_1.len() {
                    crates_1.push(Vec::new());
                }

                if !chars[char_index].is_whitespace() {
                    crates_1[stack_index].push(chars[char_index]);
                }

                char_index += 4;
                stack_index += 1;
            }
        }
    }

    // Part 1.
    for stack in crates_1.iter() {
        if let Some(top_crate) = stack.last() {
            solution.0.push(*top_crate);
        }
    }

    // Part 2.
    for stack in crates_2.iter() {
        if let Some(top_crate) = stack.last() {
            solution.1.push(*top_crate);
        }
    }
}

#[test]
fn test_05() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(5, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "SPFMVDTZT");
    assert_eq!(solution.1, "ZFSJBPRFP");
}
