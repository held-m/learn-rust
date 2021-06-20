fn main() {
    println!("Hello, world!");
    if_expression();
    short_expression();
}


fn if_expression() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("my number {}", number);
}

fn short_expression() {
    let is_five = true;
    let number = if is_five { 5 } else { 6 };

    println!("The value of number is {}", number);
}