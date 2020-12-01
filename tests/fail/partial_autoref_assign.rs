use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op(autoref)]
fn sub_assign(_lhs: &mut X, _rhs: X) { }

fn partial_subtraction() {
    let mut x = X;
    assert_eq!((), x -= X);
    assert_eq!((), x -= &X);
}

fn main() {}
