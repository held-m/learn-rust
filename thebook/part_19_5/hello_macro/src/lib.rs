#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait HelloMacro {
    fn hello_macro() {
        println!("hi from crate HelloMacro");
    }
}
