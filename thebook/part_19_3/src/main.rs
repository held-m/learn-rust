use std::io;

fn main() {
    println!("Hello, world!");
    alias_type();
    get_info();
}

fn alias_type() {
    type M1 = i32;
    let v1: M1 = 7;
    let v2: M1 = 9;
    let res = v1 + v2;
    println!("res: {}", res);
}

struct MyStruct {
    pub prop: &'static str,
}

fn get_info() {
    let mut my_str = String::new();
    io::stdin().read_line(&mut my_str).expect("Failed to read");
    check_generic("hi");
}

fn check_generic<T: ?Sized>(t: &T) {
    let p = t;
    println!("hi generic");
}
