fn main() {
    println!("Dropping a vector drops its elements");
    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
