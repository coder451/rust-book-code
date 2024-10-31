fn main() {
    println!("Ch 8.2-3 Updating a String");

    // Add to the end of a String
    let mut s = String::from("foo");
    println!("Starting string: {}", &s);
    s.push_str("bar");
    println!("After push_str(\"bar\"): {}\n", &s);

}
