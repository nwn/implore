use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(autoref)]
fn deref(lhs: &X) -> &X {
    lhs
}

fn main() {}
