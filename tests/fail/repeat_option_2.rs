use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(autoref, commutative, autoref)]
fn add(lhs: X, _rhs: &u32) -> X {
    lhs
}

fn main() {}
