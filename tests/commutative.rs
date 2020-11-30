use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Y;

#[impl_op(commutative)]
fn add(lhs: X, _rhs: Y) -> X {
    lhs
}

#[impl_op(commutative)]
fn add(_lhs: &X, _rhs: &Y) -> X {
    X
}

#[impl_op(commutative)]
fn add(_lhs: X, _rhs: &Y) -> X {
    X
}

#[impl_op(commutative)]
fn add(lhs: &X, _rhs: Y) -> X {
    *lhs
}

#[test]
fn split_addition() {
    assert_eq!(X, X + Y);
    assert_eq!(X, X + &Y);
    assert_eq!(X, &X + Y);
    assert_eq!(X, &X + &Y);
    assert_eq!(X, Y + X);
    assert_eq!(X, Y + &X);
    assert_eq!(X, &Y + X);
    assert_eq!(X, &Y + &X);
}

#[impl_op(autoref, commutative)]
fn sub(lhs: X, _rhs: &Y) -> X {
    lhs
}

#[test]
fn partial_subtraction() {
    assert_eq!(X, X - Y);
    assert_eq!(X, X - &Y);
    // assert_eq!(X, &X - Y);
    // assert_eq!(X, &X - &Y);
    assert_eq!(X, Y - X);
    assert_eq!(X, &Y - X);
    // assert_eq!(X, Y - &X);
    // assert_eq!(X, &Y - &X);
}

#[impl_op(autoref, commutative)]
fn mul(lhs: &X, _rhs: Y) -> X {
    *lhs
}

#[test]
fn partial_multiplication() {
    assert_eq!(X, X * Y);
    // assert_eq!(X, X * &Y);
    assert_eq!(X, &X * Y);
    // assert_eq!(X, &X * &Y);
    assert_eq!(X, Y * X);
    // assert_eq!(X, &Y * X);
    assert_eq!(X, Y * &X);
    // assert_eq!(X, &Y * &X);
}

#[impl_op(autoref, commutative)]
fn div(lhs: &X, _rhs: &Y) -> X {
    *lhs
}

#[test]
fn total_division() {
    assert_eq!(X, X / Y);
    assert_eq!(X, X / &Y);
    assert_eq!(X, &X / Y);
    assert_eq!(X, &X / &Y);
    assert_eq!(X, Y / X);
    assert_eq!(X, &Y / X);
    assert_eq!(X, Y / &X);
    assert_eq!(X, &Y / &X);
}
