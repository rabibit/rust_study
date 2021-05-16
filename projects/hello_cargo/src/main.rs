fn main() {
    println!("Hello, world!");
    test();
}

fn test() {
    let mut data = vec![String::from("123"), String::from("456")];
    let x = data[0].clone();
    data.push(String::from("789"));
    println!("x: {}, data[0]: {:?}", x, data);
}