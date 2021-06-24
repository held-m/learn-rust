fn main() {
    create_iterator();
    sum();
    map();
}

fn create_iterator() {
    let v1 = vec![1, 2, 3];

    let v2 = v1.clone();
    let v2 = v2.iter();

    let mut v3 = v2.clone();

    for item in v1 {
        println!("Got: {}", item);
    }

    for item in v2 {
        println!("v2: {}", item);
    }

    println!("v1: {:?}", v3.next());

    println!("v3: {:?}", v3);

    //-----------

    let mut my_counter: Counter = Counter::new();
    println!("my counter 1: {:?}", my_counter.next());
    println!("my counter 2: {:?}", my_counter.next());
    println!("my counter 3: {:?}", my_counter.next());
    println!("my counter 4: {:?}", my_counter.next());
    println!("my counter 5: {:?}", my_counter.next());
    println!("my counter 6: {:?}", my_counter.next());
    println!("my counter 7: {:?}", my_counter.next());
    println!("my counter 8: {:?}", my_counter.next());




    //-----------
}

fn sum() {
    let v1: Vec<i8> = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i8 = v1_iter.sum();
    println!("total: {}", total);
}

fn map() {
    let v1: Vec<i8> = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let v2: Vec<i8> = v1_iter.map(|x| x + 1).collect();

    println!("map: {:?}", v2);
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}



// --------------------

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else {
            None
        }
    }
}


// --------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec! [
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );

    }


    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
