
fn main() {
    create_vec();
    read_vec();
    check_own_vec();
    vec_in_for();
    create_vec_size();
}

fn create_vec() -> (){
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(2);
    v1.sort();
    println!("v1: {:?}", v1);

    let v2: Vec<&str> = vec!["1", "2", "3"];
    println!("v2: {:?}", v2);

}

fn read_vec() -> () {
    let v = vec![1,2,3,5,8,7];
    let h: &i32 = &v[2];
    println!("h: {}", h);
    print_vec(&v);

    match v.get(5) {
        Some(h1) => println!("match h: {}", h1),
        None => println!("no index"),
    }
}

fn print_vec(v: &Vec<i32>) -> () {
    println!("print vec v: {:?}", v);
}

fn check_own_vec() -> () {
    let mut v = vec![1, 2, 3, 4, 5,];
    let &ff = &v[4];

    v.push(7);
    println!("check own ff: {}", ff);

}

fn vec_in_for() -> () {
    let mut v = vec![101, 35, 88, 56];
    for i in &mut v {
        // *i += 2;
        *i = (*i + 5) * 2;
        println!("for v: {}", i);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vec() -> () {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn create_vec_size() -> () {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(10);
    v.push(12);
    v.push(14);
    v.push(15);
    v.push(16);

    let x = &v[1];
    print_index(x);
    // drop(x);

    v.push(58);

    println!("v: {:?}", v);

}

fn print_index(x: &i32) -> () {
    println!("x: {}", x)
}
