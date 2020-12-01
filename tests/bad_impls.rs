use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn pow(lhs: X, exponent: u32) -> X {
    lhs
}

#[impl_op]
fn deref(lhs: &X) -> HashMap<&'static str, Vec<u32>> {
    lhs
}

#[impl_op]
fn deref(lhs: &X) {
    lhs
}
