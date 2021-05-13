
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle1 is {}", area1(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle2 is {}", area2(rect1));

    let rect2 = Rectangle{
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect4 = Rectangle{
        width: 60,
        height: 45,
    };

    println!("The area of the rectangle3 is {}", area3(&rect2));
    println!("The area method of the rectangle  is {}", rect2.area());
    println!("rect2 is {:#?}", rect2);

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));

    let square1 = Rectangle::square(9);
    println!("square1 is {:#?}", square1);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}