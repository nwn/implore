use impl_ops_proc::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn add_assign(_lhs: &mut X, _rhs: X) { }

#[impl_op]
fn add_assign(_lhs: &mut X, _rhs: &X) { }

#[test]
fn split_addition() {
    let mut x = X;
    assert_eq!((), x += X);
    assert_eq!((), x += &X);
}

#[impl_op(autoref)]
fn sub_assign(_lhs: &mut X, _rhs: X) { }

#[test]
fn partial_subtraction() {
    let mut x = X;
    assert_eq!((), x -= X);
    // assert_eq!((), x -= &X);
}

#[test]
fn recursive_addition() {
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Uint(u32);

    #[impl_op]
    fn add_assign(lhs: &mut Uint, rhs: Uint) {
        if rhs.0 != 0 {
            add_assign(lhs, Uint(rhs.0 - 1));
            lhs.0 += 1;
        }
    }

    let mut x = Uint(4);
    assert_eq!((), x += Uint(5));
    assert_eq!(Uint(9), x);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct I32(i32);

#[impl_op(autoref)]
fn add_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 += rhs.0
}
#[impl_op(autoref)]
fn sub_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 -= rhs.0
}
#[impl_op(autoref)]
fn mul_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 *= rhs.0
}
#[impl_op(autoref)]
fn div_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 /= rhs.0
}
#[impl_op(autoref)]
fn rem_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 %= rhs.0
}
#[impl_op(autoref)]
fn bitand_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 &= rhs.0
}
#[impl_op(autoref)]
fn bitor_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 |= rhs.0
}
#[impl_op(autoref)]
fn bitxor_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 ^= rhs.0
}
#[impl_op(autoref)]
fn shl_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 <<= rhs.0
}
#[impl_op(autoref)]
fn shr_assign(lhs: &mut I32, rhs: &I32) {
    lhs.0 >>= rhs.0
}

#[test]
fn add_assign() {
    let mut x = I32(6);
    assert_eq!((), x += I32(-2));
    assert_eq!(I32(4), x);
    assert_eq!((), x += &I32(-2));
    assert_eq!(I32(2), x);
}

#[test]
fn sub_assign() {
    let mut x = I32(6);
    assert_eq!((), x -= I32(-2));
    assert_eq!(I32(8), x);
    assert_eq!((), x -= &I32(-2));
    assert_eq!(I32(10), x);
}

#[test]
fn mul_assign() {
    let mut x = I32(6);
    assert_eq!((), x *= I32(-6));
    assert_eq!(I32(-36), x);
    assert_eq!((), x *= &I32(-6));
    assert_eq!(I32(216), x);
}

#[test]
fn div_assign() {
    let mut x = I32(6);
    assert_eq!((), x /= I32(4));
    assert_eq!(I32(1), x);
    assert_eq!((), x /= &I32(4));
    assert_eq!(I32(0), x);
}

#[test]
fn rem_assign() {
    let mut x = I32(15);
    assert_eq!((), x %= I32(4));
    assert_eq!(I32(3), x);
    assert_eq!((), x %= &I32(2));
    assert_eq!(I32(1), x);
}

#[test]
fn bitand_assign() {
    let mut x = I32(0b1110);
    assert_eq!((), x &= I32(0b0111));
    assert_eq!(I32(0b0110), x);
    assert_eq!((), x &= &I32(0b1010));
    assert_eq!(I32(0b0010), x);
}

#[test]
fn bitor_assign() {
    let mut x = I32(0b0001);
    assert_eq!((), x |= I32(0b1000));
    assert_eq!(I32(0b1001), x);
    assert_eq!((), x |= &I32(0b0101));
    assert_eq!(I32(0b1101), x);
}

#[test]
fn bitxor_assign() {
    let mut x = I32(0b1011);
    assert_eq!((), x ^= I32(0b1100));
    assert_eq!(I32(0b0111), x);
    assert_eq!((), x ^= &I32(0b0101));
    assert_eq!(I32(0b0010), x);
}

#[test]
fn shl_assign() {
    let mut x = I32(6);
    assert_eq!((), x <<= I32(3));
    assert_eq!(I32(48), x);
    assert_eq!((), x <<= &I32(1));
    assert_eq!(I32(96), x);
}

#[test]
fn shr_assign() {
    let mut x = I32(28);
    assert_eq!((), x >>= I32(3));
    assert_eq!(I32(3), x);
    assert_eq!((), x >>= &I32(1));
    assert_eq!(I32(1), x);
}
