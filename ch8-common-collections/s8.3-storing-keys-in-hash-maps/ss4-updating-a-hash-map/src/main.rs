fn main() {
    println!("CH8.3-4 Updating a Hash Map");

    {
        // Overwrite a value with HashMap.insert
        println!("Overwrite test");
        use std::collections::HashMap;

        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
    
        println!("{scores:?}");
    }

    {
        // Add if not present using entry to check if there is an
        // entry already, and or_insert
        println!("\nAdd-if-not-present test");
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        let e = scores.insert(String::from("Blue"), 10);
        println!("Blue insertion result (None means no entry was present before the insertion): {:?}", e);
        let e = scores.entry(String::from("Orange"));
        println!("Result from getting a non-existent entry: {:?}", e);

        // entry gets the 
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
    
        println!("The scores hash map is now: {scores:?}");    
    }

    {
        // Update a value based on an old value - using a hash map to count 
        // occurrences
        use std::collections::HashMap;

        let text = "hello world wonderful world";
    
        let mut map = HashMap::new();
    
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    
        println!("Word counts from \"{text}\": {map:?}");
    
    }
}
