use std::slice;
use std::ops::Add;
use std::fmt;

static mut COUNTER: u32 = 0;
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        *r2 += 1;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    person.fly();
    Human::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    // fully qualified syntax
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());  

    let p = Point { x: 1, y: 3 };
    p.outline_print();

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (v1, v2) = split_at_mut(&mut v, 3);
    println!("v1: {:?}, v2: {:?}", v1, v2);

    let color = Color(1, 2, 3);
    println!("color: ({}, {}, {})", color.0, color.1, color.2);

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // use closure as parameter
    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = 
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("The list of string is: {:?}", list_of_string);
    
    // use function as parameter
    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = 
        list_of_numbers.iter().map(ToString::to_string).collect();
    println!("The list of string is: {:?}", list_of_string);

    let list_of_statuses: Vec<Status> = (2u32..22).map(Status::Value).collect();
    for (idx, val) in list_of_statuses.iter().enumerate() {
        match val {
            Status::Value(v) => println!("idx: {}, value: {}", idx, v),
            Status::Stop => (),
        }
    }

    let a = Box::new(8);
    println!("{}", *a);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(self: &Self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// using supertrait to require one trait's functionality within another trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Color(i32, i32, i32);

// using the newtype pattern to implement external traits on external types
// here using a tuple struct to avoid implement fmt::Display for Vec<T> directly, or
// we can't implement fmt::Display for Vec<T>
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}