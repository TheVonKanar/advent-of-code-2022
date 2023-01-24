pub(super) fn resolve(puzzle: String) -> (usize, usize) {
    eprintln!("{}", puzzle);
    (1, 1)
}

#[test]
fn test_01() {
    assert_eq!(resolve(crate::solvers::get_puzzle(1)), (1, 1));
}
