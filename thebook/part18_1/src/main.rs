fn main() {
    while_let();
    for_loop();
    let_statement();
    let nam: String = String::from("hi");
    println!("hi {}", nam);
    let p = 6;
    println!("p: {}", p);
}

fn while_let() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let str = vec!["Hi", "World", "Hohoho"];

    for (index, words) in str.iter().enumerate() {
        println!("index: {}, value {}", index, words);
    }
}

fn let_statement() {
    let (x, y, ..) = (1, 2, 4, 5, 7);

    println!("x: {}, y: {}", x, y);
}

fn myy() {
    let v = 6;
}

fn hi() {
    let str: Vec<u32> = vec![1, 2, 3];
}
