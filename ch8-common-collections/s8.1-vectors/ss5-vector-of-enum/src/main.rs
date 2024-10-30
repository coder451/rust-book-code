fn main() {
    // This approach works if we know the types at compile time
    // Otherwise, see traits (Ch 17.)
    println!("Using an enum to store multiple types in a vector");
    
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

    // This illustrates explicitly accessing the individual values from 
    // the enum
    let i = match row[0] {
        SpreadsheetCell::Int(i) => i,
        _ => -1
    };
    let f = match row[2] {
        SpreadsheetCell::Float(f) => f,
        _ => -1.0
    };
    let t = match &row[1] {
        SpreadsheetCell::Text(t) => String::from(t),
        _ => String::from("")
    };

    println!("{:?}", i);
    println!("{:?}", f);
    println!("{:?}", &t);
}