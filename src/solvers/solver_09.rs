use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, solution: &mut Solution, _: &mut Info) {
    let puzzle = get_puzzle(solver.index);

    let mut rope: Vec<(isize, isize)> = vec![(0, 0); 10];
    let mut tracks_1 = vec![(0, 0)];
    let mut tracks_2 = vec![(0, 0)];

    for line in puzzle.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let steps = split.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..steps {
            match direction {
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                &_ => todo!(),
            }

            for i in 1..rope.len() {
                let prev_knot = rope[i - 1];
                let diff = (
                    prev_knot.0.abs_diff(rope[i].0),
                    prev_knot.1.abs_diff(rope[i].1),
                );

                if diff.0 >= 2 || diff.1 >= 2 {
                    if diff.0 > 0 {
                        rope[i].0 += if prev_knot.0 > rope[i].0 { 1 } else { -1 };
                    }

                    if diff.1 > 0 {
                        rope[i].1 += if prev_knot.1 > rope[i].1 { 1 } else { -1 };
                    }
                }
            }

            let tail_1 = rope[1].clone();
            if !tracks_1.contains(&tail_1) {
                tracks_1.push(tail_1);
            }

            let tail_2 = rope.last().unwrap().clone();
            if !tracks_2.contains(&tail_2) {
                tracks_2.push(tail_2);
            }
        }
    }

    solution.0 = tracks_1.len().to_string();
    solution.1 = tracks_2.len().to_string();
}

#[allow(dead_code)]
fn debug_rope(rope: Vec<(isize, isize)>) {
    let mut debug = String::new();
    for y in -15..15 {
        'outer: for x in -15..15 {
            for i in 0..rope.len() {
                if rope[i] == (x, y) {
                    if i == 0 {
                        debug.push('H');
                    } else {
                        debug.push_str(i.to_string().as_str());
                    }

                    continue 'outer;
                }
            }

            debug.push('.');
        }

        debug.push('\n');
    }

    eprintln!("{}", debug);
}

#[test]
fn test_09() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(9, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "6314");
    assert_eq!(solution.1, "2504");
}
