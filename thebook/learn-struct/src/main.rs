#![allow(dead_code)]
use mylib::hh;

fn main() {

    hh::jj();
    // println!("Hello, world!");
    //
    // let mut x = build_user("Tom".to_owned(), "hhh".to_owned(), false);
    //
    // x.username.push_str(" hohoho");
    //
    // let black = Color("hh".to_owned(), 0, 0);
    // let origin = Point(0, 0, 0);
    // println!("black: {}", black.0);
    //
    // println!("u: {}, e: {}, a: {}", x.username, x.email, x.active);
    //
    //
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // rect1.set_width(88);
    // dbg!(&rect1);
    // println!("rect1 is {}", rect1.area());
    // println!("rect1 is {}", rect1.hh());



}



struct Color(String, i32, i32);
struct Point(i32, i32, i32);



fn build_user(username: String, email: String, active: bool) -> User {
    User{
        username,
        email,
        active,
    }
}

struct User {
    username: String,
    email: String,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn hh(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}


impl Rectangle {
   pub fn set_width(&mut self, width: u32){
        self.width = width;
    }
}
