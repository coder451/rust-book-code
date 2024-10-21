#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// An impl block for the struct
// Functions in the impl block are called associated functions
// If self is the first parameter they are methods.
// If not, the function is often a factory
// Multiple impl blocks can be used
impl Rectangle {
    // Function to tell if the width is positive
    // The use of self as a parameter makes it a method
    // No type is required on self
    // Use &mut self to define a mutating method
    // Just using self grabs the instance, so you can return it as 
    // something else perhaps
    // Can use field name as method name as well, disambiguate based
    // on whether name is followed by ()
    fn width(&self) -> bool {
        self.width > 0
    }

    // self parameter comes first
    // immutable borrow of other in this case
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }

    // Non-method
    // Keyword Self used for type name
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Parentheses after the width identifier means the method.
    // No parentheses means the member.
    // No automatic getters in Rust.
    // Rust does automatic referencing and dereferencing
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    if (&rect1).width() {
        println!("The rectangle has a nonzero width; it is {}", (&rect1).width);
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
    
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    {
        let square = Rectangle::square(30);
        println!("Is the rectangle square? {}", square.is_square());
    }
}