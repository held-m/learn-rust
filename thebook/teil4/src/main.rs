fn main() {

    check_mut_refs();
}

fn check_mut_refs2() {
    let mut x = "hiiii".to_owned();
    let mut r = &mut x;
    // let mut r = &mut x;
    println!("x: {}; r: {}", x, r);
    // ch(&mut x);
    // ch(&mut x);



    // x.push_str(" 1");
    // println!("X: {}", x);
     // drop(x);
    // println!("r: {}", r);
    // println!("endX: {}", x);
}

fn ch(x: &mut String){
    x.push_str(" 2");
    println!("x1: {}", x);
    drop(x);
}

fn ch1(x: &mut String) {
    x.push_str(" 3");
    println!("x3: {}", x);
}

fn check_mut_refs() {
    let x = String::from("hohoh");

    let r1 = &x;
    let r2 = &x;

    print!("r1: {}; r2: {}", r1, r2);
    print!("r1: {}", r1);

}

fn print_var(x: &mut String){
    print!("{}: {}", x, x);
}

fn checkInt() {
    let x: u32 = 56;

    let y: u32 = x;

    println!("x: {}; y: {}", x, y);
}

fn getStringObj() -> String {
    return String::from("hohoho");
}

fn getString() -> &'static str {
    return "fkljhfkgkflg";
}

fn setString(str: &str) {
    println!("str: {}", str);
}


fn someTests() {
    let x = 5;

    let y = x;

    let str5 = "ffff";
    let str6 = str5;

    let mut str8 = getString();
    setString(str8);
    let mut str8 = format!("{}{}", str8, "55555");
    str8.push_str("ssssss");
    println!("str8: {}", str8);


    let mut str9 = getStringObj();

    str9.push_str(" ffff");

    println!("str9: {}", str9);

    let str1 = String::from("hoho");

    let str2 = str1;

    println!("x: {}", x);
    println!("y: {}", y);

    println!("str5: {}", str5);
    println!("str6: {}", str6);
}