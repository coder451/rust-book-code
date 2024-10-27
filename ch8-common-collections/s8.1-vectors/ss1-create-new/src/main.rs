fn main() {
    println!("Create new");

    // Creation without storing anything requires a type
    // annotation on the left.
    let _v: Vec<i32> = Vec::new();

    // Macro is used to create with values
    let _w = vec![1, 2, 3];

    // The other form of this macro is to replicate elements using clone.
    // Beware of types that have weird clone operations.
    let _x = vec![1; 3];

    // The macro can be used to create a zero-length vector.
    // The expression provides the type.
    // The expression is evaluated; beware of the side effects of
    // that evaluation.
    let _y = vec![42; 0];
}