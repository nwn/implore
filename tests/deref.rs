use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn deref(_: &X) -> &'static str {
    "target"
}

#[test]
fn immutable_deref() {
    assert_eq!("target", &*X);
}

enum Either<T> {
    Left(T),
    Right(T),
}

#[impl_op]
fn deref<T>(either: &Either<T>) -> &T {
    match either {
        Either::Left(t) => t,
        Either::Right(t) => t,
    }
}

#[impl_op]
fn deref_mut<T>(either: &mut Either<T>) -> &mut T {
    match either {
        Either::Left(t) => t,
        Either::Right(t) => t,
    }
}

#[test]
fn deref() {
    assert_eq!(vec![12], *Either::Left(vec![12]));
    assert_eq!("right", *Either::Right(String::from("right")));
}

#[test]
fn deref_mut() {
    let mut first = Either::Left(-10);
    let mut second = Either::Right(10);
    *first += 10;
    *second -= 10;
    assert_eq!(0, *first);
    assert_eq!(0, *second);
}
