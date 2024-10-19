fn main() {
    let result = doaloop();
    println!("The result is {result}");
}

fn doaloop() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 10 {
            return counter * 2;
        }
    }
}