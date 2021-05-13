fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
    change(&mut s1);

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    let r1 = &s1;
    let r2 = &s1;
    println!("{}{}", r1, r2);

    let r3 = &mut s1;

    println!("{}", r3);

    let my_string = String::from("hello world!");
    let word = first_word(&my_string[..]);
    println!("{}", word);
    let my_string_literal = "hello world!";
    println!("{}", word);
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);
    let word = first_word(my_string_literal);
    println!("{}", word);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();

    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}