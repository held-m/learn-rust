fn main() {
    let answer = do_twice(add_one, 8);
    println!("my answer {}", answer);

    let hh = return_closure();
    println!("my return closure {} ", hh(8));
    println!("my return closure {} ", hh(8));

    let once = check_fn_once();
    println!("once1 {}", once(3));
    // println!("once1 {}", once(3));
    println!("check_fn_once {}", (check_fn_once())(5));
    println!("check_fn_once {}", (check_fn_once())(5));

    let mut mut_fn = check_fn_mut();
    println!("check fn mut {}", mut_fn(7));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn map_stuf() {
    let numbers = vec![1, 2, 3];
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn check_fn_once() -> Box<dyn FnOnce(i32) -> i32> {
    Box::new(|x| x + 2)
}

fn check_fn_mut() -> Box<dyn FnMut(i32) -> i32> {
    Box::new(|x| x - 1)
}
