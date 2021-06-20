use std::collections::HashMap;

pub fn task1() -> () {
    let v1: Vec<i32> = Vec::from([
        1, 4, 3, 6, 4, 6, 33, 5, 66, 7, 33, 23, 34, 56, 7, 8, 4, 3, 45, 54, 33,
    ]);

    println!("mean: {}", get_mean(&v1));
    println!("median: {}", get_median(&v1));
    println!("mode: {}", get_mode(&v1));
}

fn get_mean(v: &Vec<i32>) -> f64 {
    // let mut v2 = v;
    let mut sum = 0;
    for item in v {
        sum += item;
    }
    let sum: f64 = sum as f64;
    let len: f64 = v.len() as f64;
    sum / len
}

fn get_median(v: &Vec<i32>) -> i32 {
    let mut v1 = v.clone();
    v1.sort();
    v1[v1.len() / 2]
}

fn get_mode(v: &Vec<i32>) -> i32 {

    let mut hash: HashMap<i32, i32> = HashMap::new();

    for &item in v.iter() {
        let count = hash.entry(item).or_insert(1);
        *count += 1;
    }

    let mut max_value = 0;
    let mut max_number = 0;

    for (key, value) in hash {
        if max_value < value {
            max_value = value;
            max_number = key;
        }
    }
    max_number
}


