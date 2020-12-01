use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(not-an-identifier)]
fn mul(lhs: X, rhs: X) -> X {
    lhs
}

fn main() {}
