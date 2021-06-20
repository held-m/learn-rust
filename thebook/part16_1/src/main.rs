use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

fn main() {

    // my_thread();
    // my_move();
    // my_message();
    // my_multiple_values();
    multiple_producer();
}

fn my_thread() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

}


fn my_move() {
    let v = vec![1, 2, 3];

    let str: String = String::from("hohohohoh");

    println!("my v still alive? {:?}", v);
    println!("my v still alive? {:?}", str);
    println!("my v still alive? {:?}", str);

    let handle = thread::spawn(move || {
        // println!("Here's a vector: {:?}", &v);
        println!("My string: {:?}", str);
    });

    handle.join().unwrap();
    println!("my v still alive? {:?}", v);

}


fn my_message() {
    let (tx, rx) = mpsc::channel();

    let handler = thread::spawn(move || {
        let val: String = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv();

    println!("Got: {}", received.unwrap());
}

fn my_multiple_values() {
    let (tx, rx) = mpsc::channel();

   thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("nevetlan"),
            String::from("hohohoh"),
            ];

       for val in vals {
           tx.send(val).unwrap();
           thread::sleep(Duration::from_secs(1));
       }
    });

    for received in rx {
        println!("Got: {}", received);
        println!("dddddd");
    }


    println!("dddddd");
}

fn multiple_producer() {
    let (tx, rx) = mpsc::channel();
    let tx1: Sender<String> = tx.clone();
    let tx2: Sender<String> = tx1.clone();
    thread::spawn(move || {
        thread_say_hello(vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ], tx);
    });

    thread::spawn(move || {
        thread_say_hello(vec![
            String::from("more"),
            String::from("messgaes"),
            String::from("for"),
            String::from("you"),
        ], tx1);
    });

    thread::spawn(move || {
        thread_say_hello(vec![
            String::from("more1"),
            String::from("messgaes1"),
            String::from("for1"),
            String::from("you1"),
        ], tx2);
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn thread_say_hello<T>(vals: Vec<T>, sender: Sender<T>) {
    for val in vals {
        sender.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}