
use std::io;

fn main() {
    loop {
        if ! ask() { return }
    }
}

fn ask() -> bool {
    println!("Enter the temperature as value type where value is a \
    decimal and type is F or C. Input q to quit");

    let mut t = String::new();
        
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");

    let binding = String::clone(&t);
    let input = binding.trim();
    println!("The input was {input}");
    if input == "q" {
        println!("Exiting because q specified");
        return false
    };

    let tokens = tokenize(&input);
    if ! tokens.0 {
        println!("Did not understand input {input} - try again");
        return true
    }

    let _t: f64 = match tokens.1.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Could not understand temperature {input} - try \
            again.");
            return true
        }
    };
    true

}

fn tokenize(_t: &str) -> (bool, &str, &str) {
    (true, "0", "C")
}
