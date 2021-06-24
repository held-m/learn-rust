fn main() {
    println!("Hello, world!");
    another_function(6, 8);

    let y = {
        let x = 3;
        x + 1
    };

    println!("my y = {}", y);

    let k = number(8);

    println!("hey get k = {}", k);

}

fn another_function(x: i32, y: i32) {
    println!("Hello,  another world {} - {}", x, y);
}

fn number(x: i32) -> i32 {
    x * 5
}