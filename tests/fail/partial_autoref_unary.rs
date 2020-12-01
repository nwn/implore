use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(autoref)]
fn neg(rhs: X) -> X {
    rhs
}

fn partial_negation() {
    assert_eq!(X, -X);
    assert_eq!(X, -&X);
}

fn main() {}
