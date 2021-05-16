use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    //  uses the automatic dereferencing feature  
    // to dereference the Rc<RefCell<T>> to the inner RefCell<T> value
    
    // let mut tmp = value.borrow_mut();
    // *tmp += 10;
    // the same as the following line
    *value.borrow_mut() += 10;

    println!("a after = {:?}", *a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}