use hello_macro::HelloMacro;
use hello_macro_derive::check;
use hello_macro_derive::HelloMacro;

// #[check()]
fn check_attr_macro() {
    println!("check_attr_macro");
}

fn main() {
    // println!("Hello, world!");
    let v1: Vec<i32> = vec![5, 6, 8, 3];
    println!("hi vec: {:?}", v1);

    WaitHellloMacro::hello_macro();

    check_attr_macro();
}

#[derive(HelloMacro)]
struct WaitHellloMacro;

// impl HelloMacro for WaitHellloMacro {}
