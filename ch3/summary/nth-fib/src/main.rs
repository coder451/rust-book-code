use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check input and convert to i64
    let mut v: Vec<i64> = Vec::new();
    let mut verbose = false;
    for (i, arg) in args.iter().enumerate() {
        if i == 0 { continue }
        if arg.starts_with("--") {
            if arg == "--verbose" {
                verbose = true;
                println!("Verbose output requested");
                continue
            }
            else {
                println!("Unrecognized option: {arg} - giving up");
                return
            }
        }

        let n: i64 = match arg.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not understand the entry {i}: {arg} \
                - giving up");
                return
            }
        };
        v.push(n);
    }

    if v.len() == 0 {
        if verbose {
            println!("Nothing to do");
        }
        return
    }

    v.sort();

    let mut fib0 = 0;
    let mut fib1 = 1;

    let mut f: Vec<u64> = Vec::new();
    let mut fib_order: i64 = 1;
    'outer_loop: for fib_index in &v {
        if *fib_index == 0 {
            f.push(0);
            if verbose {
                println!("Fib 0 is 0");
            }
            continue
        }
        else if *fib_index == 1 {
            f.push(1);
            if verbose { 
                println!("Fib 1 is 1");
            }
            continue
        }

        // At this point the requested index is at least 2.
        // Loop the Fibonacci calc until the index gets to the current.
        loop {
            let fib_temp = fib1;

            // Overflow guard
            if fib1 > u64::MAX - fib0 {
                if verbose {
                    println!("Requested order too large for u64 \
                    - giving up at {fib_order}: {fib1}, still smaller than \
                    requested order {0}", *fib_index);
                }
                break 'outer_loop 
            } 
            fib1 = fib1 + fib0;
            fib0 = fib_temp;
            fib_order += 1;
            if verbose {
                println!("... Fib {fib_order}: {fib1}");
            }
            if fib_order == *fib_index {
                // fib1 is the requested value
                if verbose {
                    println!("... is one we seek: {fib_index}");
                }
                break
            }
        }
        if verbose {
            println!("Fib {fib_index} requested is {fib1}");
        }
        f.push(fib1);
    }
    for (i, elem) in v.iter().enumerate() {
        if i < f.len() {
            println!("{elem}: {0}", f[i]);
        }
        else {
            println!("{elem}: too large");
        }
    }
}
