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

    let ss = [
        "", " ", " a", "a ", " a ",
        "  ", " ab", "ab ", " ab ",
        "   ", "  abc", "abc  ", "  abc  ",
    ];
    for item in ss {
        let s1 = String::from(item);
        let w1 = first_word(&s1);
        println!("First word for \"{}\": \"{}\"", s1, w1)
    }

}


fn first_word(s: &String) -> &str {
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
