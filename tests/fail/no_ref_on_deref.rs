use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn deref_mut(x: X) -> &X {
    &X
}

fn main() {}
