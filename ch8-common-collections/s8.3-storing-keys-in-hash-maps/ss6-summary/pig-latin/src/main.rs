fn main() {
    println!("Pig Latin");

    let cases = vec![
        "Hello there!",
        "My name is Elvis."
    ];

    for case in cases {
        let t = translate(case);
        println!("{}: {}", case, t);
    }
}

fn translate(text: &str) ->String {
    text.to_string()
}
