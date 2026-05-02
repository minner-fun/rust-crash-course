use function::*;

#[test]
fn test_mul() {
    assert_eq!(mul(1, 0), 0);
    assert_eq!(mul(0, 1), 0);
    assert_eq!(mul(10, 2), 20);
    assert_eq!(mul(13, 7), 91);

    // commutativity
    assert_eq!(mul(7, 13), mul(13, 7));

    // identity and zeros
    assert_eq!(mul(42, 1), 42);
    assert_eq!(mul(0, 12345), 0);

    // boundary: multiplying by 1 preserves value
    assert_eq!(mul(u32::MAX, 1), u32::MAX);
}

#[test]
fn test_add_plus() {
    assert_eq!(add_plus(1, 2), (3, false));
    assert_eq!(add_plus(50, 51), (101, true));
    assert_eq!(add_plus(0, 0), (0, false));
    assert_eq!(add_plus(100, 0), (100, false));
    assert_eq!(add_plus(99, 2), (101, true));
}

#[test]
fn test_pure_add() {
    // This function doesn't return anything, so we just call it to ensure it runs without error.
    let res = pure_add(3, 4);
        // Since pure_add prints to stdout, we could capture the output if needed, but here we just ensure it runs.
    assert_eq!(res, ()); // pure_add returns the unit type `()`
}

#[test]
fn test_div() {
    assert_eq!(div(0, 1), 0);
    assert_eq!(div(10, 2), 5);
    assert_eq!(div(13, 7), 1);
    // assert_eq!(div(2, 0), 1);
}

// Verify that division by zero panics (documented behavior for integer division)
#[test]
#[should_panic]
fn test_div_by_zero_panics() {
    let _ = div(1, 0);
}
