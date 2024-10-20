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
// In this enum, Dog is an enum variant, while Cat is 
// a "struct-like enum variant" because it has named constructor
// parameters. It looks like a struct, and uses {} in its 
// constructor.
enum Animal {
    Dog(String, f64),
    Cat { name: String, weight: f64 },
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

    // This constructs a mutable reference to a Dog
    let mut a = Animal::Dog("Cocoa".to_string(), 37.2);
    // This changes the Animal to be a Cat
    a = Animal::Cat { name: "Spotty".to_string(), weight: 2.7 };
    // This makes another cat, using the opposite order of named parameters
    let b = Animal::Cat { weight: 2.7, name: "Spotty".to_string() };

}
