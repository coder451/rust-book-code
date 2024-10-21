#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associated function - factory making a square
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle::square(30);

    println!("rect: {rect:?}");
}