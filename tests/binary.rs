#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_ops_proc::impl_op2]
fn add(lhs: X, _rhs: X) -> X {
    lhs
}

#[impl_ops_proc::impl_op2]
fn add(_lhs: &X, _rhs: &X) -> X {
    X
}

#[impl_ops_proc::impl_op2]
fn add(_lhs: X, rhs: &X) -> X {
    *rhs
}

#[impl_ops_proc::impl_op2]
fn add(lhs: &X, _rhs: X) -> X {
    *lhs
}

#[test]
fn split_addition() {
    assert_eq!(X, X + X);
    assert_eq!(X, X + &X);
    assert_eq!(X, &X + X);
    assert_eq!(X, &X + &X);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct I32(i32);

#[impl_ops_proc::impl_op2]
fn add(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 + rhs.0)
}
#[impl_ops_proc::impl_op2]
fn sub(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 - rhs.0)
}
#[impl_ops_proc::impl_op2]
fn mul(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 * rhs.0)
}
#[impl_ops_proc::impl_op2]
fn div(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 / rhs.0)
}
#[impl_ops_proc::impl_op2]
fn rem(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 % rhs.0)
}
#[impl_ops_proc::impl_op2]
fn bitand(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 & rhs.0)
}
#[impl_ops_proc::impl_op2]
fn bitor(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 | rhs.0)
}
#[impl_ops_proc::impl_op2]
fn bitxor(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 ^ rhs.0)
}
#[impl_ops_proc::impl_op2]
fn shl(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 << rhs.0)
}
#[impl_ops_proc::impl_op2]
fn shr(lhs: I32, rhs: I32) -> I32 {
    I32(lhs.0 >> rhs.0)
}

#[test]
fn add() {
    assert_eq!(I32(4), I32(6) + I32(-2));
    // assert_eq!(I32(4), &I32(6) + I32(-2));
    // assert_eq!(I32(4), I32(6) + &I32(-2));
    // assert_eq!(I32(4), &I32(6) + &I32(-2));
}

#[test]
fn sub() {
    assert_eq!(I32(4), I32(2) - I32(-2));
    // assert_eq!(I32(4), &I32(2) - I32(-2));
    // assert_eq!(I32(4), I32(2) - &I32(-2));
    // assert_eq!(I32(4), &I32(2) - &I32(-2));
}

#[test]
fn mul() {
    assert_eq!(I32(-12), I32(2) * I32(-6));
    // assert_eq!(I32(-12), &I32(2) * I32(-6));
    // assert_eq!(I32(-12), I32(2) * &I32(-6));
    // assert_eq!(I32(-12), &I32(2) * &I32(-6));
}

#[test]
fn div() {
    assert_eq!(I32(-2), I32(-11) / I32(4));
    // assert_eq!(I32(-2), &I32(-11) / I32(4));
    // assert_eq!(I32(-2), I32(-11) / &I32(4));
    // assert_eq!(I32(-2), &I32(-11) / &I32(4));
}

#[test]
fn rem() {
    assert_eq!(I32(-3), I32(-11) % I32(4));
    // assert_eq!(I32(-3), &I32(-11) % I32(4));
    // assert_eq!(I32(-3), I32(-11) % &I32(4));
    // assert_eq!(I32(-3), &I32(-11) % &I32(4));
}

#[test]
fn bitand() {
    assert_eq!(I32(0x0100), I32(0x1101) & I32(0x0110));
    // assert_eq!(I32(0x0100), &I32(0x1101) & I32(0x0110));
    // assert_eq!(I32(0x0100), I32(0x1101) & &I32(0x0110));
    // assert_eq!(I32(0x0100), &I32(0x1101) & &I32(0x0110));
}

#[test]
fn bitor() {
    assert_eq!(I32(0x1101), I32(0x1100) | I32(0x0101));
    // assert_eq!(I32(0x1101), &I32(0x1100) | I32(0x0101));
    // assert_eq!(I32(0x1101), I32(0x1100) | &I32(0x0101));
    // assert_eq!(I32(0x1101), &I32(0x1100) | &I32(0x0101));
}

#[test]
fn bitxor() {
    assert_eq!(I32(0x1001), I32(0x1100) ^ I32(0x0101));
    // assert_eq!(I32(0x1001), &I32(0x1100) ^ I32(0x0101));
    // assert_eq!(I32(0x1001), I32(0x1100) ^ &I32(0x0101));
    // assert_eq!(I32(0x1001), &I32(0x1100) ^ &I32(0x0101));
}

#[test]
fn shl() {
    assert_eq!(I32(40), I32(5) << I32(3));
    // assert_eq!(I32(40), &I32(5) << I32(3));
    // assert_eq!(I32(40), I32(5) << &I32(3));
    // assert_eq!(I32(40), &I32(5) << &I32(3));
}

#[test]
fn shr() {
    assert_eq!(I32(2), I32(19) >> I32(3));
    // assert_eq!(I32(2), &I32(19) >> I32(3));
    // assert_eq!(I32(2), I32(19) >> &I32(3));
    // assert_eq!(I32(2), &I32(19) >> &I32(3));
}
