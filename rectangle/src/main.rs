fn main() {
    let rec1 = (30,80);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rec1)
    );
}

fn area (rec: (u32,u32)) -> u32 {
    rec.0 * rec.1
}
