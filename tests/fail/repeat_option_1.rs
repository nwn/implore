use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(autoref, autoref)]
fn neg(lhs: &X) -> X {
    *lhs
}

fn main() {}
