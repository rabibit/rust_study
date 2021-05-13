use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x; // use keyword "move" to take the ownership

    // can't use x here, because its ownership is token away and moved into closure
    // println!("{:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

// fn simulated_expensive_caluculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_caluculation(intensity);
    
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

struct Cacher<T, W>
where
    T: Fn(W) -> W, W: Copy,
{
    calculation: T,
    value: Option<W>,
}

impl<T, W> Cacher<T, W>
where
    T: Fn(W) -> W, W: Copy,
{
    fn new(calculation: T) -> Cacher<T, W> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: W) -> W {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}