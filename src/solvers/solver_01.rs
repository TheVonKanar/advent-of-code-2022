pub(super) fn resolve(puzzle: String) -> (usize, usize) {
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
    
    (loads[0], loads[0] + loads[1] + loads[2])
}

#[test]
fn test_01() {
    assert_eq!(resolve(crate::solvers::get_puzzle(1)), (69281, 201524));
}
