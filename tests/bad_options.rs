use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(auto)]
fn not(lhs: X) -> X {
    lhs
}

#[impl_op(autoref, autoref)]
fn neg(lhs: X) -> X {
    lhs
}

#[impl_op(autoref, commutative, autoref)]
fn add(lhs: X, rhs: &X) -> X {
    lhs
}

#[impl_op(autoref)]
fn sub(lhs: X, rhs: X) -> X {
    lhs
}

#[impl_op(not-an-identifier)]
fn mul(lhs: X, rhs: X) -> X {
    lhs
}

#[impl_op(autoref)]
fn deref(lhs: &X) -> &X {
    lhs
}

#[impl_op(commutative)]
fn index(lhs: &X, idx: usize) -> &X {
    lhs
}
