#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // tells me exactly what is going on here in the program output!
        height: 50 * scale,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // {:?} says use the Debug output format
    println!("rect1 is {:?}", rect1);

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
