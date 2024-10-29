fn main() {
    println!("Iterating over values in a vector");
    {
        // for i in &v
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
    }
    {
        // Changing vector values
        // The vector must be mutable
        let mut v = vec![100, 32, 57];
        println!("After mutation");
        // The reference to its values must be mutable
        for i in &mut v {
            // Using dereference operator to operate on what it refers
            // to. See Chapter 15.
            *i += 50;
            println!("{i}");
        }
    }
}