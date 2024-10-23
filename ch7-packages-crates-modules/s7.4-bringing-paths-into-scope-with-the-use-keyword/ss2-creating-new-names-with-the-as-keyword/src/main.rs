pub mod MyIo {
    pub struct Result {
        pub n: i32
    }
}
pub mod YourIo {
    pub struct Result {
        pub n: i32
    }
}

use MyIo::Result;
use YourIo::Result as YourIoResult;


fn function1() -> Result {
    Result{n: 0}
}

fn function2() -> YourIoResult {
    YourIoResult{n: 0}
}

fn main() {
    println!("Creating new names with the as keyword");
}
