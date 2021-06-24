fn main() {
        println!("Hello, world!");
    try_loop1();
    try_while();
    try_for();
    try_for_range_rev();
    try_for_range_iter();
}

fn try_loop1() {
    let mut counter = 0;

    let result =loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

    };

    println!("The result is {}", result);
}

fn try_loop2() {
    let counter = 0;

    let result =loop {
        let counter = counter + 1;

        if counter == 12 {
            break counter * 2;
        }
        println!("counter: {}", counter);
    };

    println!("The result is {}", result);
}

fn try_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -=1;
    }

    println!("while liftoff");
}

fn try_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the element is: {}", element);
    }
}

fn try_for_range_rev() {
    // with iter doesn't work
    for number in (1..6).rev() {
        println!("{}!", number);
    }
}

fn try_for_range_iter() {
    // whaaaaaaaaaaat
    for number in (7..1).count() {
        println!("iter {}!", number);
    }
}