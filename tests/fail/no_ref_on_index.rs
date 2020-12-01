use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn index(x: X, y: u32) -> &X {
    &X
}

fn main() {}
