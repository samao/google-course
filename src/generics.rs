use tracing::info;

pub fn generics_run() {
    info!("generics mode run");
    info!("even is {:?}", pick(10, ('A', 19), ('B', 20)));
    info!("odd is {:?}", pick(13, ('C', 19), ('D', 20)));

    info!("From impl {:?}, {:?}", Foo::from(32), Foo::from(true));
}

fn min<T: Ord>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

#[test]
fn test_min() {
    assert_eq!(min(1, 2), 1);
    assert_eq!(min(2, 1), 1);
    assert_eq!(min('a', 'b'), 'a');
    assert_eq!(min('b', 'a'), 'a');
    assert_eq!(min("goood", "bad"), "bad");
}

fn pick<T>(n: i64, even: T, odd: T) -> T {
    if n % 2 == 0 { even } else { odd }
}

#[derive(Debug)]
struct Foo(String);

impl From<i32> for Foo {
    fn from(value: i32) -> Self {
        Foo(format!("from i32 = {} to Foo", value))
    }
}

impl From<bool> for Foo {
    fn from(value: bool) -> Self {
        Foo(format!("from bool = {} to Foo", value))
    }
}
