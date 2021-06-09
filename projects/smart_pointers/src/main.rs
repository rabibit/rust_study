use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("Hello, world! b = {}", b);
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    // let y = &x;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // the next two statements are the sanme
    assert_eq!(5, *y); // 自动解引用
    assert_eq!(5, *(y.deref())); // 手动解引用

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // the same as hello(&(*m)[..]);
    hello(&String::from("wang"));

    let u = User{ name: "Raymond" };
    let y = MySmartPointer::new(u);

    y.name(); // 自动解引用
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

struct MySmartPointer<T>(T);

impl<T> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct User {
    name: &'static str
}

impl User {
    fn name(&self) {
        println!("Hello {:?}", self.name);
    }
}