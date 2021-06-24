use std::io;
use rand::Rng;

fn main() {

    let mut hh = String::new();
    io::stdin()
        .read_line(&mut hh)
        .expect("dfdgd");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Hello, world!");

}
