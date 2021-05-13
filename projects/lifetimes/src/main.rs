use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let result = longest("hello", "wang");
    println!("the longest string is: {}", result);

    let result = longest_with_an_announcement("hello", "wang", "I love Rust");
    println!("the longest string is: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where
        T: Display, T: Copy
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }