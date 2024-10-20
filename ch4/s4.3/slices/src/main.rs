fn main() {
    println!("String slices");

    {                              // 01234567890
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
        println!("{hello}, {world}.")
    }

}
