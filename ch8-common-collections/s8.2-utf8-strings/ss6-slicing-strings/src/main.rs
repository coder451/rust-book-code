use std::env;
fn main() {
    println!("Ch8.2-6 Slicing Strings");
    {
        let hello = "Здравствуйте";

        let s = &hello[0..4];

        println!("{s}");

    }

    {
        let hello = "Здравствуйте";

        // This causes a panic:
        // let s = &hello[0..1];
        // By passing an index to this program you can cause it to panic.
        // If nothing is passed, or it is not an unsigned integer, the
        // whole string is returned.

        let args: Vec<String> = env::args().collect();
        let idx = match args.get(1) {
            Some(arg) =>  {
                let i: usize = match arg.parse() {
                    Ok(num) => {
                        println!("Got index {num}");
                        num
                    },
                    Err(_) => {
                        println!("Got apparent non-index \"{}\"; using whole string", arg);
                        hello.len()
                    },
                };
                i
            },
            _ => {
                println!("No parameters supplied; using the whole string");
                hello.len()
            },
        };

        let s = &hello[0..idx];
        println!("{}", s);
    }
}
