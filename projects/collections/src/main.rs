use std::{collections::HashMap, vec};
fn main() {
    let a_t = vec![1, 2];
    println!("Hello, world! {:?}, test is {}", a_t, test());
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("Hello, world! {:?}, test is {}", v, test());
    
    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(9) {
        Some(third) => println!("The ninth element is {}", third),
        None => println!("There is no ninth element."),
    }

    for i in &v {
        println!("{}", i);
    }

    let a = "abcdef";
    let b = &a[1..4];
    let c = String::from("abcdef");
    let d = c[1..].to_string();
    let e = [0x30u8, 0x31, 0x32, 0x61];
    let  a_to_str = unsafe { std::str::from_utf8_unchecked(&e)}; 
    println!("result is {}", a_to_str);
    // match a_to_str {
    //     Ok(i) => println!("result is: {}", i),
    //     Err(_) => println!("dfasdfads"),
    // }

    let data = "initial contents";
    let mut s = data.to_string();
    println!("{}", s);
    let s2 = "_jjjjj";
    s.push_str("_kkkkk");
    println!("{}", s);
    s.push_str(s2);
    println!("{}", s);
    println!("s2 is: {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is: {}", s3);

    let s1 = String::from("wang");
    let s2 = String::from("you");
    let s3 = String::from("quan");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("the result of format is: {}", s);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"),50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Green")).or_insert(40);
    println!("");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    let score = scores.get(&String::from("Blue"));
    println!("Blue score is: {:#?}", score);
    match score {
        Some(i) => println!("score is: {}", i),
        None => (),
    }
    let score = scores.get(&String::from("Blue")).unwrap();
    println!("Blue score is: {:#?}", score);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}

fn test() -> u32 {
    // return 3 + 2
    5 + 2
}