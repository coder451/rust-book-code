fn main() {
    println!("String slices");

    {                              // 01234567890
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
        println!("{hello}, {world}.")
    }

    {
        let s = String::from("hello");
        println!("These are the same:");
        let slice = &s[0..2];
        println!("0..2: {slice}");
        let slice = &s[..2];
        println!(" ..2: {slice}");
    }


    {
        let s = String::from("hello");

        let len = s.len();
        
        println!("These are the same:");
        let slice = &s[3..len];
        println!("3..{len}: {slice}");
        let slice = &s[3..];
        println!("3.. : {slice}");
    }

}
