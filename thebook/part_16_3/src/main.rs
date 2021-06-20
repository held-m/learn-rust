use std::sync::{Mutex, MutexGuard, Arc};
use std::thread;

fn main() {
    // api_mutex();
    mutex_between_threads();
}

fn api_mutex() {
    let m: Mutex<i32> = Mutex::new(5);

    {
        let mut num: MutexGuard<i32> = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn mutex_between_threads() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}