fn main() {
    println!("Ch8.2-5 Indexing into Strings");

    {
        let s1 = String::from("hello");

        // Error
        // let h = s1[0];

        let _h = s1.chars().nth(0);

        // or

        let _h = s1.bytes().nth(0);
    
    }

    {
        let hindi = String::from("नमस्ते");
        println!("{}", hindi);

        let a = hindi.chars().nth(0);
        match a {
            Some(a) => println!("Position 0, char: {}", a),
            None => println!("No char is position 0"),
        }

        let a = hindi.bytes().nth(0);
        match a {
            Some(a) => println!("Position 0, byte: {}", a),
            None => println!("No byte is position 0"),
        }

        println!("Iterate over the bytes");
        let mut i = 0;
        for c in hindi.bytes() {
            println!("{}: {}", i, c);
            i += 1;
        }

        println!("Iterate over the chars");
        let mut i = 0;
        for c in hindi.chars() {
            println!("{}: {}", i, c);
            i += 1;
        }

        use unicode_segmentation::UnicodeSegmentation;
        println!("Iterate over the graphemes - is_extended = false");
        let mut i = 0;
        for c in hindi.graphemes(false) {
            println!("{}: {}", i, c);
            i += 1;
        }
        println!("Iterate over the graphemes - is_extended = true");
        let mut i = 0;
        for c in hindi.graphemes(true) {
            println!("{}: {}", i, c);
            i += 1;
        }



    }
}
