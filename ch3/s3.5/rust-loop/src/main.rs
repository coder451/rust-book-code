fn main() {
    let mut n: i32 = 0;
    loop {
        n += 1;
        const MAX: i32 = 10;
        if n == MAX { break };
        println!("again!");
    }
}