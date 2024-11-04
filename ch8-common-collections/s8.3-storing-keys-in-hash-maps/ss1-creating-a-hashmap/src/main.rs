fn main() {
    println!("Ch8.3-1 Creating a HashMap");

    // Convention is to "use" the struct name and so not need
    // qualification i the code
    use std::collections::HashMap;

    // The types used in the map are inferred from its use later in 
    // the file.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // The default output prints the keys second
    println!("{:?}", scores);
}
