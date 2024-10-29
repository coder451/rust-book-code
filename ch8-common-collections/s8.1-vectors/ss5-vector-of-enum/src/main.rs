fn main() {
    // This approach works if we know the types at compile time
    // Otherwise, see traits (Ch 17.)
    println!("Using an enum to store multiple types in a vector");
    
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", &row[0]);
    println!("{:?}", &row[1]);
    println!("{:?}", &row[2]);
}