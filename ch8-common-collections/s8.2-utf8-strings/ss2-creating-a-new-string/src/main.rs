fn main() {
    println!("Creating a new String");

    // Empty string
    let s = String::new();
    println!("An empty string: \"{}\"", s);

    // New string with initial content
    let s1 = "abc".to_string();
    println!("String from literal with to_string: \"{}\"", &s1);

    let data = "cde";
    let s2 = data.to_string();
    println!("String from data with to_string: \"{}\"", &s2);

    let s3 = String::from("efg");
    println!("String from literal with String::from: \"{}\"", &s3);

    // Other examples:
    let hello = String::from("السلام عليكم");
    println!("Ex.: {}", &hello);
    let hello = String::from("Dobrý den");
    println!("Ex.: {}", &hello);
    let hello = String::from("Hello");
    println!("Ex.: {}", &hello);
    let hello = String::from("שלום");
    println!("Ex.: {}", &hello);
    let hello = String::from("नमस्ते");
    println!("Ex.: {}", &hello);
    let hello = String::from("こんにちは");
    println!("Ex.: {}", &hello);
    let hello = String::from("안녕하세요");
    println!("Ex.: {}", &hello);
    let hello = String::from("你好");
    println!("Ex.: {}", &hello);
    let hello = String::from("Olá");
    println!("Ex.: {}", &hello);
    let hello = String::from("Здравствуйте");
    println!("Ex.: {}", &hello);
    let hello = String::from("Hola");
    println!("Ex.: {}", &hello);

}
