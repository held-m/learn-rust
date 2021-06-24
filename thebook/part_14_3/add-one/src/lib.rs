use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn check_rand() {
    let x: i32 = rand::random();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
