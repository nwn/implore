use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn deref(lhs: &X) {
    lhs
}

fn main() {}
