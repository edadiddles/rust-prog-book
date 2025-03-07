#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let scale = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30*scale),
        height: 80
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    dbg!(&rect1);
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
}

