fn main() {
    let c = CustomSmartPointer {
        data: String::from("text1"),
    };

    let d = CustomSmartPointer {
        data: String::from("text2"),
    };

    drop(d);

    let a = CustomSmartPointer {
        data: String::from("text3"),
    };

    let b = CustomSmartPointer {
        data: String::from("text4"),
    };


    println!("CustomSmartPointer created");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
