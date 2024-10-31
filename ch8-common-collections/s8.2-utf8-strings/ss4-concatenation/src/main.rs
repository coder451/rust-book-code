fn main() {
    println!("Concatenation with the + Operator or the format! Macro");

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        println!("We start with two strings: \"{}\" and \"{}\"", s1, s2);

        // To concatenate them, the first is passed to the add function
        // relinquishing ownership, because the first parameter of the
        // add function for String takes ownership.
        // So s1 cannot be used after the add.
        // The second parameter of add takes a reference to a slice.
        // We can pass a reference to a String there, because Rust can
        // coerce from one reference to another using deref coercion.
        // This arrangement means that strings are not being copied
        // here; the internal vector of s1 is used to accommodate the
        // contents of s2 (the vector might grow) and then that vector
        // is moved into s3.
        let s3 = s1 + &s2; 
        println!("After concatenation: \"{}\"\n", s3);

        // This would borrow a moved value
        // println!("{}", s1);
    
    }

    {
        println!("With more strings, using + becomes annoying");
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        println!("Combine \"{}\", \"{}\", \"{}\"", s1, s2, s3);
    
        let s = s1 + "-" + &s2 + "-" + &s3;

        println!("Resulting in \"{}\"\n", s);
    }

    {
        println!("A better approach uses the macro format!(\"{{s1}}-{{s2}}-{{s3}}\")");
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
    
        let s = format!("{s1}-{s2}-{s3}");
        println!("Resulting in \"{}\"\n", s);
    }
}
