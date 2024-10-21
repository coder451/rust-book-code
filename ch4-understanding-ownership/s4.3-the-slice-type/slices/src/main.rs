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

    {    let ss = [
            "", " ", " a", "a ", " a ",
            "  ", " ab", "ab ", " ab ",
            "   ", "  abc", "abc  ", "  abc  ",
        ];
        for item in ss {
            let s1 = String::from(item);
            // Uses "deref coercion"
            let w1 = first_word_str(&s1);
            println!("First word for \"{}\": \"{}\"", s1, w1)
        }
        for item in ss {
            let w1 = first_word_str(item);
            println!("First word for \"{}\": \"{}\"", item, w1)
        }
    }
    {
        let s = "abc";
        let w = first_word_str(s);
        println!("First word for \"{}\": \"{}\"", s, w)
    }

    {
        // Another kind of slice
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}


fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut have_word = false;
    let mut start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if ! have_word {
            if item != b' ' {
                have_word = true;
                start = i;
            }
        }
        else {
            if item == b' ' {
                return &s[start..i];
            }
        }
    }

    if have_word { &s[start..] } else { &s[0..0] }
}
