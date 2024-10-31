fn main() {
    println!("Ch 8.2-3 Updating a String");

    {
        // Add to the end of a String
        let mut s = String::from("foo");
        println!("Starting string: {}", &s);
        s.push_str("bar");
        println!("After push_str(\"bar\"): {}\n", &s);
    }
    {
        // push_str uses a reference &str
        println!("push_str uses a reference");
        let mut s1 = String::from("foo");
        println!("Starting with {}", &s1);
        let s2 = "bar";
        println!("Append s2: {}", &s2);
        s1.push_str(s2);
        println!("Now the string has been augmented: {}", &s1);
        println!("What we appended is still there: s2 is {s2}\n");    
    }

    {
        println!("The push method takes a single character");
        let mut s = String::from("LO");
        println!("Start with s = \"{}\"", &s);
        s.push('L');
        println!("After adding one character with push: \"{}\"", s);
    }

}
