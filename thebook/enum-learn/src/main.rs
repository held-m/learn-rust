fn main() {
    println!("L: {}", value_in_cents(Coin::Nickel));
    check_match();
    println!("{}",check_placeholder(55));
}
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
//
// impl Message {
//     fn call(&self) {
//         dbg!(self);
//         // method body would be defined here
//     }
// }
//
//
// fn use_enum(){
//
//     let m = Message;
//     m::Write(String::from("hello"));
//     m::Message::ChangeColor(5,8,9);
//     m.call();
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn check_match()
{
    let five = Some(8);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);
}

fn check_placeholder(v: u32) -> u32 {

    match v {
        1 => 1,
        3 => 2,
        5 => 3,
        7 => 4,
        _ => 0,
    }
}