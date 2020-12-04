use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn deref_mut(x: &mut X) -> X {
    X
}

#[impl_op]
fn deref_mut(x: X) -> &mut X {
    &mut X
}

#[impl_op]
fn deref_mut(x: &mut X) -> &X {
    &X
}

#[impl_op]
fn deref_mut(x: &X) -> &mut X {
    &mut X
}

fn main() {}
