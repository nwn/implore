use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn not(lhs: X) -> X {
    lhs
}

#[impl_op]
fn not(_lhs: &X) -> X {
    X
}

#[test]
fn split_not() {
    assert_eq!(X, !X);
    assert_eq!(X, !&X);
}

#[impl_op]
fn neg(rhs: X) -> X {
    rhs
}

#[test]
fn partial_negation() {
    assert_eq!(X, -X);
    // assert_eq!(X, -&X);
}

#[test]
fn recursive_negation() {
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Int(i32);

    #[impl_op]
    fn neg(rhs: Int) -> Int {
        if rhs.0 > 0 {
            Int(neg(Int(rhs.0 - 1)).0 - 1)
        } else if rhs.0 < 0 {
            Int(neg(Int(rhs.0 + 1)).0 + 1)
        } else {
            rhs
        }
    }

    assert_eq!(Int(-4), -Int(4));
    assert_eq!(Int(4), -Int(-4));
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct I8(i8);

#[impl_op(autoref)]
fn neg(rhs: &I8) -> I8 {
    I8(-rhs.0)
}
#[impl_op(autoref)]
fn not(rhs: &I8) -> I8 {
    I8(!rhs.0)
}

#[test]
fn neg() {
    assert_eq!(I8(6), -I8(-6));
    assert_eq!(I8(6), -&I8(-6));
}

#[test]
fn not() {
    assert_eq!(I8(105), !I8(-106));
    assert_eq!(I8(105), !&I8(-106));
}
