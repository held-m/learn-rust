use std::time::SystemTime;

pub fn time_exec_fn<R, V>(f: fn(V) -> R, text: V, fn_name: &str) -> R {
    let start_fn = SystemTime::now();
    let result: R = f(text);
    let end_fn = SystemTime::now();

    let exec_time = end_fn
        .duration_since(start_fn)
        .expect("Clock may have gone backwards");
    // let fn_info = dbg!(f);
    println!("time execuition {}: {:?}", fn_name, exec_time);

    result
}
