use std::ops::Deref;

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("HELLO, {}", name);
}

#[test]
fn test_deference() {
    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(6, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
