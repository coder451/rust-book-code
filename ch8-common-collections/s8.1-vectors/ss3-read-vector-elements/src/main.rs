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

    // This would cause a panic
    //let _s = v[6];

    // In the following code, the print statement uses an immutable
    // reference to the first element. So the line that updates the
    // vector would cause a compile error
    let mut _w = vec![1, 2, 3, 4, 5];
    let w0 = &_w[0];
    //w.push(6);
    println!("{w0}");


}