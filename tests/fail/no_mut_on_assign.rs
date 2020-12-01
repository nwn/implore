use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn add_assign(x: &X, y: u32) {}

fn main() {}
