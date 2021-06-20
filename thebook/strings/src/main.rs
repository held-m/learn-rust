#![allow(unused)]

use std::collections::HashMap;
use std::option::Option;


use crate::tasks::task1::task1;
use crate::tasks::task2::task2;
use crate::tasks::task3::task3;

mod tasks;

fn main() {
    // change_str1();
    // chars_str();
    // hh();
    // check_hash();
    // try_hash();
    // read_hash();
    // upd_hash();
    // play_vec();
    // count_words();
    // task1();
    // task2();
    // task1();
    // task2();
     task3();
}

fn change_str() {
    let mut s1 = String::from("foo");
    let mut s2 = "bar".to_string();
    s1.push_str(&s2);
    println!("s2 is {}", s2);
    s2.push_str("rrrr");
    println!("s2 is {}", s2);
}

fn change_str1() {
    let s1 = String::from("foo");
    let s2 = "bar".to_string();
    let s3 = s1 + &s2;
    println!("s3: {}", s3);
}

fn chars_str() {
    let s = "трям";
    let chars = s.chars();
    for c in chars {
        println!("char: {}", c);
    }
}

fn hh() -> () {
    let a = "ddd".to_string();
    let b = "hhhh";
    let c = &a;

    let mut x = "гегегеегегге".to_owned();
    let mut y = "приатеомооьд".to_string();
    // let mut x = &y;
    println!("{}", y);
    println!("{}", x);
}

fn check_hash() -> () {
    let mut scores = HashMap::new();

    let team1: &str = "red";
    let team2: &str = "blue";

    scores.insert(team1, 10);
    scores.insert(team2, 50);
    println!("hoho: {:?}", scores);
}

fn try_hash() -> () {
    let color: String = String::from("red");
    let question: String = String::from("question");

    let mut list = HashMap::new();
    list.insert(question, &color);
    println!("list: {:?}", list);
    println!("color: {}", color);
}

fn read_hash() -> () {
    let mut scores: HashMap<String, &str> = HashMap::new();

    scores.insert(String::from("Blue"), "10");
    scores.insert(String::from("Yellow"), "50");

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    if let Some(h) = score {
        println!("ff: {}", h);
    }

    println!("score: {:?}", score);
}

fn upd_hash() -> () {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("score blue: {:?}", scores.get("Blue"));

    scores.insert(String::from("Blue"), 55);
    println!("score blue: {:?}", scores.get("Blue"));
}

fn play_vec() -> () {
    // let v = vec![1, 2, 3, 4, 5];
    //
    // let third: i32 = v[2];
    // println!("The third element is {}", third);
    //
    // match v.get(4) {
    //     Some(i) => println!("The third element is {}", i),
    //     None => println!("There is no third element."),
    // }
}

fn count_words() -> () {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}



