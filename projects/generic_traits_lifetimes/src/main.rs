fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Hello, world! the largest number is: {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Hello, world! the largest number is: {}", largest);

    let a = 8;
    let b = &a;
    print_number(*b);
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_t(&number_list);
    println!("Hello, world! the largest number is: {}", result);

    let char_list = vec!['w', 'y', 'q'];
    let result = largest_t(&char_list);
    println!("Hello, world! the largest char is: {}", result);

    let point_a = Point::<i32>::new(8, 9);
    println!("{}-{}", point_a.x, point_a.y);
    point_a.get();
    let point_a: Point<f32> = Point::create(8.0, 9.0);
    println!("the distance is: {}", point_a.distance_from_orign());

    let point_str = Point::<String>::new(String::from("hello"), String::from("world"));
    println!("{}-{}", point_str.x, point_str.y);

    let point_str = Point::create(String::from("hello"), String::from("world"));
    point_str.get();
    Point::<String>::get(&point_str);
    println!("{}-{}", point_str.x, point_str.y);

    test(&Dog);
    test(&Cat);
    test(&9);
    test_static(&9);

    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn print_number(number: i32) {
    println!("{}", number);
}

fn largest_t<T>(list: &[T]) -> &T where T: PartialOrd {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T1>  Point<T1> {
    fn create(x: T1 , y: T1) -> Point<T1> {
        Point{ x, y }
    }
}

impl  Point<i32> {
    fn new(x: i32 , y: i32) -> Self {
        Point{ x, y }
    }

    fn get(self: &Self) {
        println!("the value is: {}-{}", self.x, self.y)
    }
}

impl Point<f32> {
    fn distance_from_orign(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl  Point<String> {
    fn new(x: String , y: String) -> Point<String> {
        Point{ x, y }
    }
    
    fn get(&self) {
        println!("the value is: {}-{}", self.x, self.y)
    }
}

trait Say {
    fn say(&self);
}

struct Cat;
struct Dog;

impl Say for Cat {
    fn say(&self) {
        println!("Meow!");
    }
}

impl Say for Dog {
    fn say(&self) {
        println!("WangWang!");
    }
}

impl Say for i32 {
    fn say(&self) {
        println!("I am {}", self);
    }
}

fn test(animal: &dyn Say) {
    animal.say()
}

fn test_static(animal: &impl Say) {
    animal.say()
}

struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}