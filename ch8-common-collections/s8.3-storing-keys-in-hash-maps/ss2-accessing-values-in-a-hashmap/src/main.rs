fn main() {
    println!("Ch8.3-2 Accessing Values in a HashMap");

    // Convention is to "use" the struct name and so not need
    // qualification i the code
    use std::collections::HashMap;

    // The types used in the map are inferred from its use later in 
    // the file.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // The default output prints the keys second
    println!("{:?}", scores);

    // New for subsection 2

    // Make a string containing the desired team name
    let team_name = String::from("Blue");

    // Access the score.
    // Using "get" returns an Option<&i32>, because it
    // refers to the value in the map. We don't want to 
    // refer to the value in the map directly so we use 
    // "copied" to get an integer of our own, still wrapped
    // in an Option<i32>. Then we use"unwrap_or" to simply
    // get the value, or -1 if it does not exist.
    let score = scores.get(&team_name)
        .copied()
        .unwrap_or(-1);
    println!("The score for {team_name} is {score}");

    let team_name = String::from("unknown");
    let score = scores.get(&team_name)
        .copied()
        .unwrap_or(-1);
    println!("The score for {team_name} is {score}");

    println!();
    println!("Iterating over the map");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
