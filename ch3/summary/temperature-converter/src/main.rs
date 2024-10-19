
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

    let t: f64 = match tokens.1.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Could not understand temperature {input} - try \
            again.");
            return true
        }
    };

    let (unit, other_unit) = match tokens.2 {
        "c"=> ("C", "F"),
        "C"=> ("C", "F"),
        "f"=> ("F", "C"),
        "F"=> ("F", "C"),
        _ => {
            println!("Did not understand the unit in {input}");
            return false
        }
    };

    println!("Result: {0:.1} {1}", t_c(t, unit), other_unit);

    true

}

fn tokenize(t: &str) -> (bool, &str, &str) {
    let tokens: Vec<&str> = t.split_whitespace().collect();
    if tokens.len() != 2 {
        (false, t, t)
    }
    else {
        (
            true, 
            // We know the size of the vector
            tokens.get(0).expect("unexpected"), 
            tokens.get(1).expect("unexpected")
        )
    }
}

fn t_c(t: f64, unit: &str) -> f64 {
    if unit == "C" {
        (t / 5.0) * 9.0 + 32.0
    }
    else {
        (t - 32.0) * 5.0 / 9.0
    }
}
