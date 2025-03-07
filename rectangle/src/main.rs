struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1: Rectangle = Rectangle {
        width: 30,
        height: 80
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rec1)
    );
}

fn area (rec: Rectangle) -> u32 {
    rec.width * rec.height
}
