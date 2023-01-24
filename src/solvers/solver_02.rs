pub(super) fn resolve() -> (usize, usize) {
    (2, 2)
}

#[test]
fn test_02() {
    assert_eq!(resolve(), (2, 2));
}
