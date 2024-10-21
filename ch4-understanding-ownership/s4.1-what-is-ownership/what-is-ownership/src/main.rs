fn main() {
    println!("What is ownership?");

    {
        let s = String::from("hello");

        // s is immutable.
        // This would try to mutate it:
        // s.push_str(", world") // error
        println!("{s}");
    }

    {
        let mut s = String::from("hello");

        // Mutation is OK for this string:
        s.push_str(", world");

        println!("{s}");

        // s goes out of scope after its last use.
        // At that point, "drop" is called automatically.
    }

    {
        // With no memory allocation, these are just copies
        let x = 5;
        let y = x;
        println!("{x}, {y}");

        // The String version is different because the String
        // contains allocated data on the heap:
        let s1 = String::from("hello");
        let s2 = s1;

        // Here s1 is no longer valid for use:
        // println!("{s1}"); // error
        // It is not valid for use because println! needs to borrow
        // from it, but the memory is now owned by s2.
        // But s2 is valid:
        println!("{s2}");
    }

    {
        // You can make a deep copy with clone
        let s1 = String::from("hello");
        let s2 = s1.clone();
    
        println!("s1 = {s1}, s2 = {s2}");
    
    }


}
