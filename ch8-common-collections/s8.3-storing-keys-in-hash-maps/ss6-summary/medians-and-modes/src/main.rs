fn main() {
    println!("Medians and modes");
    let result = compute_stats();
    println!("{:?}", result);
}

#[derive(Debug)]
pub struct Result {
    median: i64,
    mode: i64,
}

pub fn compute_stats() -> Result {
    Result{median: 0, mode: 0,}
}
