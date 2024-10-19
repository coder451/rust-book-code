fn main() {
    let s1 = String::from("hello");

    // The call explicitly passes a reference, there is no
    // automatic conversion to a reference. The reference does not
    // take ownership, and it is immutable. Creating a reference is
    // called _borrowing_.
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // The following would be an error - see below
    // change(&s1);

    // If we have a mutable string, it can be changed in a function
    // that takes a mutable reference as a parameter
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 is {s2}");

    // Not allowed to have any other reference if there is a mutable
    // reference:
    let s2a = &mut s2; // OK

    // Not OK - attempt to borrow when there is already
    // a mutable reference
    // let s2b = &s2;

    // This is not OK - two mutable references
    // let s2c = &mut s2; 
    println!("{s2a}");

    // The same things works in reverse:
    let mut s3 = String::from("hello");
    s3.push_str(", world");
    let s3ra = &s3;
    let s3rb = &s3;
    // let s3rc = &mut s3; // - error
    println!("{s3}; {s3ra}; {s3rb}");

    {
        // Here is the interesting thing:
        // This code looks like it has both immutable and mutable
        // references. But r1 and r2 are not used after r3 is
        // introduced, so it's OK. In effect, the scope of a variable 
        // ends after its last use.
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");
        // variables r1 and r2 will not be used after this point
    
        let r3 = &mut s; // no problem
        println!("{r3}");

        // If this were added, r3 would be invalid
        // println!("{r2}");
    
    }

    // Uncomment this and the related function to see an error about
    // a dangling reference
    // {
    //     let reference_to_nothing = dangle();
    //     println!("{reference_to_nothing}");
    // }
    
}

// The function signature specifies that it takes an 
// immutable reference
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.



// This function is erroneous because it tries to mutate the reference
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// This function is OK
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Erroneous function returns reference to local variable
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }