
use std::io;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        loop {
            if ! ask() { return }
        }
    }
    else {
        let tokens = collect_args(args);
        compute(&tokens, false);
    }
}

fn collect_args(args: Vec<String>) -> String {
    let arg1 = args.get(1)
        .expect("Get command line arg at index 1");
    let mut tokens = String::clone(arg1);
    if args.len() == 3 {
        let arg2 = args.get(2)
            .expect("Can get command ine arg at index 2");
        tokens += " ";
        tokens += arg2;
    }
    tokens
}

fn compute(input: &str, verbose: bool)  -> bool {
    let tokens = tokenize(&input);
    if ! tokens.0 {
        if verbose {
            println!("Did not understand input {input} - try again");
        }
        return true
    }

    let t: f64 = match tokens.1.parse() {
        Ok(num) => num,
        Err(_) => {
            if verbose {
                println!("Could not understand temperature {input} - try \
                again.");
            }
            return true
        }
    };

    let (unit, other_unit) = match tokens.2 {
        "c"=> ("C", "F"),
        "C"=> ("C", "F"),
        "f"=> ("F", "C"),
        "F"=> ("F", "C"),
        _ => {
            if verbose {
                println!("Did not understand the unit in {input}");
            }
            return false
        }
    };

    if verbose {
        println!("Result: {0:.1} {1}", t_c(t, unit), other_unit);
    }
    else {
        println!("{0:.1}{1}", t_c(t, unit), other_unit);
    }
    true

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

    compute(input, true)
}

fn tokenize(t: &str) -> (bool, &str, &str) {
    let tokens: Vec<&str> = t.split_whitespace().collect();
    if tokens.len() == 2 {
        (
            true, 
            // We know the size of the vector
            tokens.get(0).expect("unexpected"), 
            tokens.get(1).expect("unexpected")
        )
    }
    else if tokens.len() == 1 {
        let tok = tokens.get(0).expect("unexpected");
        let len = tok.len();
        if len < 2 {
            return (false, t, t)
        }
        (
            true, 
            &tok[..(len - 1)], 
            &tok[(len - 1)..len]
        )
    }
    else {
        (false, t, t)
    }
}

fn t_c(t: f64, unit: &str) -> f64 {
    if unit == "C" {
        (t / 5.0) * 9.0 + 32.0
    }
    else if unit == "F" {
        (t - 32.0) * 5.0 / 9.0
    }
    else {
        panic!("Unexpected unit");
    }
}
