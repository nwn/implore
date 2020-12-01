use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(auto)]
fn not(lhs: X) -> X {
    lhs
}

fn main() {}
