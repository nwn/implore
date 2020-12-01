use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(autoref)]
fn sub(lhs: X, _rhs: X) -> X {
    lhs
}

fn main() {}
