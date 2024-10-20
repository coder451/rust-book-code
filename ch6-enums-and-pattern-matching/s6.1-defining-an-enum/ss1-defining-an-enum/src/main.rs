enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// In this enum, Dog is an enum variant, while Cat is 
// a "struct-like enum variant" because it has named constructor
// parameters. It looks like a struct, and uses {} in its 
// constructor.
enum Animal {
    Dog(String, f64),
    Cat { name: String, weight: f64 },
}

// An enum where none of the constructors have parameters is said to
// be a fieldless enum. The variants can be tuple-like, struct-like
// or unit-like.
enum Fieldless {
    Tuple(),
    Struct{},
    Unit,
}


// Unit-only enum can contains constants, apparently
enum Fbb {
    Foo = 3,
    Bar = 2,
    Baz = 1,
}


fn main() {
    println!("Enums and pattern matching");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    // Method on an enum
    let m = Message::Write(String::from("hello"));
    m.call();


    // This constructs a mutable reference to a Dog
    let mut a = Animal::Dog("Cocoa".to_string(), 37.2);
    // This changes the Animal to be a Cat
    a = Animal::Cat { name: "Spotty".to_string(), weight: 2.7 };
    // This makes another cat, using the opposite order of named parameters
    let b = Animal::Cat { weight: 2.7, name: "Spotty".to_string() };

    let tupleInstance = Fieldless::Tuple();
    let structInstance = Fieldless::Struct{};
    let UnitInstance = Fieldless::Unit;

    let foo = Fbb::Foo;

}
