use std::fmt::Display;

fn main() {
    let number_list: Vec<i32> = vec![5,15,35,8,9,12,45,68,4,5,3,2,0];

    let char_list: Vec<char> = vec!['f', 'g', 'd', 'r', 't', 'y', 'F'];

    println!("largest number: {:?}", largest_32(&number_list));
    println!("largest char: {:?}", largest_char(&char_list));




    let integer: Point<i32, i32> = Point{ x: 5, y: 10 };
    println!("int x: {}, y: {}", integer.x, integer.y);

    let float: Point<f32, f32> = Point{ x: 5.2, y: 25.3 };
    println!("f x: {}, y: {}", float.x, float.y);
    println!("get dist: {}", float.dist_from_f32());

    let num: Point<i32, f32> = Point{ x: 5, y: 12.5 };
    println!("num x: {}, y: {}", num.x, num.y);

    println!("get x: {}", num.x());
    println!("get y: {}", num.y());
    println!("dist: {}", num.dist_from_i32());

    let p1: Point<i32, f32> = Point { x: 5, y: 10.5};
    let p2: Point<&str, char> = Point { x: "Hi", y: 'g' };

    let p3 = p1.mixup(p2);
    println!("mixup x: {}, y:{}", p3.x, p3.y);

    println!("largest num: {}", largest(&number_list));
    println!("largest num: {}", largest(&char_list));

    let pair = Pair::new(5, 8);
    println!("x: {} y: {}", pair.x, pair.y);


}

fn largest_32(list: &[i32]) -> i32 {
    let mut largest: i32 = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<i32, f32> {
    fn dist_from_i32(&self) -> f32 {
        self.x as f32 + self.y
    }
}

impl Point<f32, f32> {
    fn dist_from_f32(&self) -> f32 {
        self.x as f32 + self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}