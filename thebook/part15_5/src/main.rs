use std::rc::{ Rc, Weak };
use std::cell::{RefCell, Ref};
use crate::List::{ Cons, Nil };
use std::collections::LinkedList;
use std::borrow::{BorrowMut, Borrow};

fn main() {
    tree();

    // memory_leak();

    // let value = Rc::new(RefCell::new(5));
    //
    // let a: Rc<List> = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    //
    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    //
    //
    //
    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // *value.borrow_mut() +=10;
    // println!("c after = {:?}", c);

}
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum List1 {
    Cons(i32, RefCell<Rc<List1>>),
    Nil,
}


impl List1 {
    fn tail(&self) -> Option<&RefCell<Rc<List1>>> {
        match self {
            List1::Cons(_, item) => Some(item),
            List1::Nil => None,
        }
    }
}

fn memory_leak() {

    let a = Rc::new(List1::Cons(5, RefCell::new(Rc::new(List1::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());


    let b = Rc::new(List1::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    println!("a next item = {:?}", a.tail());

}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn tree() {
    let leaf = Rc::new(Node{
       value: 3,
       parent: RefCell::new(Weak::new()),
       children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value:5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("{:?}", &branch);
    println!("{:?}", branch.children.borrow_mut()[0].value);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("parent value {:?}", leaf.parent.borrow().upgrade().);

}
