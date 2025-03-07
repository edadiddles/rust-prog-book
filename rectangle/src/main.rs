#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}
fn main() {
    let scale = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30*scale),
        height: 80
    };
    let rect2: Rectangle = Rectangle {
        width: 50,
        height: 90
    };
    let rect3: Rectangle = Rectangle {
        width: 10,
        height: 30
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    dbg!(&rect1);
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    println!("rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

