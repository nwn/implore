use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(autoref)]
fn sub(lhs: X, _rhs: &X) -> X {
    lhs
}

fn partial_subtraction() {
    assert_eq!(X, X - X);
    assert_eq!(X, X - &X);
    assert_eq!(X, &X - X);
    assert_eq!(X, &X - &X);
}

#[impl_op(autoref)]
fn mul(_lhs: &X, rhs: X) -> X {
    rhs
}

fn partial_multiplication() {
    assert_eq!(X, X * X);
    assert_eq!(X, X * &X);
    assert_eq!(X, &X * X);
    assert_eq!(X, &X * &X);
}

fn main() {}
