fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("The value of spaces is '{}'", spaces);
    let spaces = spaces.len();
    println!("Now the value of spaces is {spaces}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    let x = 2.1; // f64
    println!("x is {x}");
    let y: f32 = 3.1; // f32
    println!("y is {y}");
}