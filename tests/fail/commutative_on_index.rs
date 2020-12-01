use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(commutative)]
fn index(lhs: &X, idx: usize) -> &X {
    lhs
}

fn main() {}
