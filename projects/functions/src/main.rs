fn main() {
    let x = plus_one(5);
    println!("Hello, world! the x is: {}", x);

    if x != 0 {
        println!("True");
    } else {
        println!("False");
    }

    let mut counter = 0;
    let result = loop {
        println!("again and again");
        counter += 1;
        if counter == 100 {
            break counter;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..9).rev() {
        println!("{}", number);
    }

    println!("LIFTOFF!!!");

}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}