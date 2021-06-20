use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    // check_move();
}

fn generate_workout(intensity: u32, random_number: u32) {

    // let expensive_result = simulated_expensive_calculation(intensity);

    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num|{
       println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });


    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number ==  3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }
//
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

struct Cacher<T, I, V>
    where
        T: Fn(I) -> V
{
    calculation: T,
    value: HashMap<I, V>,
}

impl<T, I, V> Cacher<T, I, V>
    where
        T: Fn(I) -> V,
        I: Eq + Hash + Clone,
        V: Clone
{
    fn new(calculation: T) -> Cacher<T, I, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: I) -> V {

        let v = self.value.entry(arg.clone()).or_insert((self.calculation)(arg));
        v.clone()
     }
}

fn check_move() {
    let x = vec![1, 2, 3];

    let equal_to_x = |z| z == x;

    println!("can't use x if add move before |z|: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

}