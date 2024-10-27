fn main() {
    println!("8.1 ss2 Updating a vector");

    // The type can be implicit because of the types present in the
    // update. The compiler will warn of mismatched types.
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("{v:?}");
}
