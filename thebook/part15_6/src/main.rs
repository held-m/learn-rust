use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let leaf2 = Rc::clone(&leaf);
    let leaf3 = Rc::downgrade(&leaf);
    let _leaf4 = Rc::clone(&leaf);
    let _leaf5 = Rc::clone(&leaf);

    println!("leaf1 paarent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf1 strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // drop(leaf3);
    println!(
        "2leaf1 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );

    println!(
        "3leaf1 strong = {}, weak = {}",
        Weak::strong_count(&leaf3),
        Weak::weak_count(&leaf3),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        println!(
            "branch1 strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf2-1 strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch2 strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf2 strong = {}, weak = {}, parent_weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
            Weak::weak_count(&leaf.parent.borrow_mut()),
        );
    }

    println!("leaf3 parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf4 strong  = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

struct Joap {
    value: i32,
    dfdf: i64,
}
