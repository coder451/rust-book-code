fn main() {
    println!("Ch8.2-7 Iterating Over Strings");

    {
        let hindi = String::from("नमस्ते");
        println!("{}", hindi);

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
