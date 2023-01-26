pub(super) fn resolve(puzzle: String) -> (usize, usize, &'static str) {
    let mut score_1: usize = 0;
    let mut score_2: usize = 0;
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

    (score_1, score_2, "")
}

#[test]
fn test_02() {
    let solution = resolve(crate::solvers::get_puzzle(2));
    assert_eq!(solution.0, 11906);
    assert_eq!(solution.1, 11186);
}
