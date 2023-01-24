pub(super) fn resolve() -> (usize, usize) {
    (1, 1)
}

#[test]
fn test_01() {
    assert_eq!(resolve(), (1, 1));
}
