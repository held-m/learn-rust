use std::ops::Mul;

fn main() {
    let my_point = Point { x: 1, y: 0 } * Point { x: 2, y: 4 };

    println!("my_point: {:?}", my_point);

    let human = Human {};

    Pilot::fly(&human);
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self) {
        println!("Hi from pilot");
    }
}

trait Wizard {
    fn fly(&self) {
        println!("Hi from Wizard");
    }
}

struct Human;

impl Pilot for Human {}
impl Wizard for Human {}
