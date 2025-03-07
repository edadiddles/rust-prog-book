#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30*scale),
        height: 80
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    dbg!(&rect1);
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
}

fn area (rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
