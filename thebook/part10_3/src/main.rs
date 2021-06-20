fn main() {
    println!("Hello, world!");

    let str1: &str = "ggggg";
    let str2: &str = "gigigi";
    let res: &str = longest(&str1, &str2);
    println!("res: {}", res);
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}