fn main() {
    println!("Read vector elements");

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let sixth: Option<&i32> = v.get(5);
    match sixth {
        Some(sixth) => println!("The sixth element is {sixth}"),
        None => println!("There is no sixth element."),
    }

}