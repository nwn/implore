use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn neg(x: &mut X, y: u32) {}

#[impl_op]
fn mul(x: &X) {}

#[impl_op]
fn add_assign(x: &mut X, y: u32, z: bool) {}

#[impl_op]
fn index_mut(x: &mut X, y: u32, z: bool) -> &mut X {}

#[impl_op]
fn deref() {}

fn main() {}
