use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn index(x: &X, y: u32) -> X {
    X
}

#[impl_op]
fn index(x: X, y: u32) -> &X {
    &mut X
}

#[impl_op]
fn index(x: &mut X, y: u32) -> &X {
    &X
}

#[impl_op]
fn index(x: &X, y: u32) -> &mut X {
    &mut X
}

fn main() {}
