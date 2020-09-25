use impl_ops_proc::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn add(lhs: X, _rhs: X) -> X {
    lhs
}

#[impl_op]
fn add(_lhs: &X, _rhs: &X) -> X {
    X
}

#[impl_op]
fn add(_lhs: X, rhs: &X) -> X {
    *rhs
}

#[impl_op]
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

#[impl_op(autoref)]
fn sub(lhs: X, _rhs: &X) -> X {
    lhs
}

#[test]
fn partial_subtraction() {
    assert_eq!(X, X - X);
    assert_eq!(X, X - &X);
    // assert_eq!(X, &X - X);
    // assert_eq!(X, &X - &X);
}

#[impl_op(autoref)]
fn mul(_lhs: &X, rhs: X) -> X {
    rhs
}

#[test]
fn partial_multiplication() {
    assert_eq!(X, X * X);
    // assert_eq!(X, X * &X);
    assert_eq!(X, &X * X);
    // assert_eq!(X, &X * &X);
}

#[test]
fn recursive_remainder() {
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Uint(u32);

    #[impl_op]
    fn rem(lhs: Uint, rhs: Uint) -> Uint {
        if lhs.0 >= rhs.0 {
            rem(Uint(lhs.0 - rhs.0), rhs)
        } else {
            lhs
        }
    }

    assert_eq!(Uint(3), Uint(15) % Uint(4));
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct I32(i32);

#[impl_op(autoref)]
fn add(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 + rhs.0)
}
#[impl_op(autoref)]
fn sub(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 - rhs.0)
}
#[impl_op(autoref)]
fn mul(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 * rhs.0)
}
#[impl_op(autoref)]
fn div(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 / rhs.0)
}
#[impl_op(autoref)]
fn rem(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 % rhs.0)
}
#[impl_op(autoref)]
fn bitand(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 & rhs.0)
}
#[impl_op(autoref)]
fn bitor(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 | rhs.0)
}
#[impl_op(autoref)]
fn bitxor(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 ^ rhs.0)
}
#[impl_op(autoref)]
fn shl(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 << rhs.0)
}
#[impl_op(autoref)]
fn shr(lhs: &I32, rhs: &I32) -> I32 {
    I32(lhs.0 >> rhs.0)
}

#[test]
fn add() {
    assert_eq!(I32(4), I32(6) + I32(-2));
    assert_eq!(I32(4), &I32(6) + I32(-2));
    assert_eq!(I32(4), I32(6) + &I32(-2));
    assert_eq!(I32(4), &I32(6) + &I32(-2));
}

#[test]
fn sub() {
    assert_eq!(I32(4), I32(2) - I32(-2));
    assert_eq!(I32(4), &I32(2) - I32(-2));
    assert_eq!(I32(4), I32(2) - &I32(-2));
    assert_eq!(I32(4), &I32(2) - &I32(-2));
}

#[test]
fn mul() {
    assert_eq!(I32(-12), I32(2) * I32(-6));
    assert_eq!(I32(-12), &I32(2) * I32(-6));
    assert_eq!(I32(-12), I32(2) * &I32(-6));
    assert_eq!(I32(-12), &I32(2) * &I32(-6));
}

#[test]
fn div() {
    assert_eq!(I32(-2), I32(-11) / I32(4));
    assert_eq!(I32(-2), &I32(-11) / I32(4));
    assert_eq!(I32(-2), I32(-11) / &I32(4));
    assert_eq!(I32(-2), &I32(-11) / &I32(4));
}

#[test]
fn rem() {
    assert_eq!(I32(-3), I32(-11) % I32(4));
    assert_eq!(I32(-3), &I32(-11) % I32(4));
    assert_eq!(I32(-3), I32(-11) % &I32(4));
    assert_eq!(I32(-3), &I32(-11) % &I32(4));
}

#[test]
fn bitand() {
    assert_eq!(I32(0x0100), I32(0x1101) & I32(0x0110));
    assert_eq!(I32(0x0100), &I32(0x1101) & I32(0x0110));
    assert_eq!(I32(0x0100), I32(0x1101) & &I32(0x0110));
    assert_eq!(I32(0x0100), &I32(0x1101) & &I32(0x0110));
}

#[test]
fn bitor() {
    assert_eq!(I32(0x1101), I32(0x1100) | I32(0x0101));
    assert_eq!(I32(0x1101), &I32(0x1100) | I32(0x0101));
    assert_eq!(I32(0x1101), I32(0x1100) | &I32(0x0101));
    assert_eq!(I32(0x1101), &I32(0x1100) | &I32(0x0101));
}

#[test]
fn bitxor() {
    assert_eq!(I32(0x1001), I32(0x1100) ^ I32(0x0101));
    assert_eq!(I32(0x1001), &I32(0x1100) ^ I32(0x0101));
    assert_eq!(I32(0x1001), I32(0x1100) ^ &I32(0x0101));
    assert_eq!(I32(0x1001), &I32(0x1100) ^ &I32(0x0101));
}

#[test]
fn shl() {
    assert_eq!(I32(40), I32(5) << I32(3));
    assert_eq!(I32(40), &I32(5) << I32(3));
    assert_eq!(I32(40), I32(5) << &I32(3));
    assert_eq!(I32(40), &I32(5) << &I32(3));
}

#[test]
fn shr() {
    assert_eq!(I32(2), I32(19) >> I32(3));
    assert_eq!(I32(2), &I32(19) >> I32(3));
    assert_eq!(I32(2), I32(19) >> &I32(3));
    assert_eq!(I32(2), &I32(19) >> &I32(3));
}
