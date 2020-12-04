use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn deref(x: &X) -> X {
    X
}

#[impl_op]
fn deref(x: X) -> &X {
    &mut X
}

#[impl_op]
fn deref(x: &mut X) -> &X {
    &X
}

#[impl_op]
fn deref(x: &X) -> &mut X {
    &mut X
}

fn main() {}
