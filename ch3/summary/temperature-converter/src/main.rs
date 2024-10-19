
use std::io;

fn main() {
    loop {
        if ! ask() { return }
    }
}

fn ask() -> bool {
    println!("Enter the temperature as value type where value is a \
    decimal and type is F or C");

    let mut t = String::new();
        
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");

    println!("The input was {t}");
    let input = String::clone(&t);
    if input.trim() == "q" { 
        println!("Exiting because q specified");
        return false
    };

    let tokens = tokenize(&input);

    let _t: f64 = match tokens.0.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Could not understand temperature {input} - try \
            again, or q to quit");
            return true
        }
    };
    true

}

fn tokenize(_t: &String) -> (&str, &str) {
    ("0", "C")
}
