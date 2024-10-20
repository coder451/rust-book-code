enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("Enums and pattern matching");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

}
