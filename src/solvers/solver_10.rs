use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, solution: &mut Solution, _: &mut Info) {
    let puzzle = get_puzzle(solver.index);

    let mut cycles: Vec<isize> = Vec::new();
    let mut x = 1isize;
    let mut signal = 0;
    let mut crt = String::new();

    for line in puzzle.lines() {
        cycles.push(0);
        let split: Vec<&str> = line.split(' ').collect();
        if split[0] == "addx" {
            cycles.push(split[1].parse().unwrap());
        }
    }

    let mut i: isize = 0;
    for cycle in cycles {
        let cursor = i % 40;
        i += 1;

        if (i - 20) % 40 == 0 {
            signal += i * x;
        }

        if cursor == 0 {
            crt.push('\n');
        }

        crt.push(if cursor.abs_diff(x) <= 1 { '#' } else { '.' });

        x += cycle;
    }

    solution.0 = signal.to_string();
    solution.1 = crt;
}

#[test]
fn test_10() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(10, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "15020");
    assert_eq!(
        solution.1,
        "\n####.####.#..#..##..#....###...##..###..
#....#....#..#.#..#.#....#..#.#..#.#..#.
###..###..#..#.#....#....#..#.#..#.#..#.
#....#....#..#.#.##.#....###..####.###..
#....#....#..#.#..#.#....#....#..#.#....
####.#.....##...###.####.#....#..#.#...."
    );
}
