use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn index_mut(x: &mut X, y: u32) -> X {
    X
}

#[impl_op]
fn index_mut(x: X, y: u32) -> &mut X {
    &mut X
}

#[impl_op]
fn index_mut(x: &mut X, y: u32) -> &X {
    &X
}

#[impl_op]
fn index_mut(x: &X, y: u32) -> &mut X {
    &mut X
}

fn main() {}
