use is_vowel::IsRomanceVowel;

fn main() {
    println!("Pig Latin");

    let cases = vec![
        "Hello",
        "+Hello",
        "!Hello",
        "Hello there!",
        "Hello the=re!",
        "My name is Elvis.",
        "œ©ªŠŒÄ",
        "this;that",
        "this; that"
    ];

    for case in cases {
        println!("\n\n* Case: {case}");
        let t = translate(case);
        println!("Result for {}: {}", case, t);
    }
}

// Iterate through the char in the text.
// Pass blanks through unchanged.
// Pass words that contain non-alpha, except a single central hyphen unchanged.
// A word for translation must start at the beginning of the string, or after 
// a space or punctuation .,;:?! Any non-alpha, non-punctuation character
// switches off the transition to alpha words until a space is seen.
// For words that contain only alpha and at most one hyphen, 
// apply the translation:
//   1st char is vowel-like: add hay
//   1st char is not vowel-like: add fay
fn translate(text: &str) -> String {
    enum Status {
        InPassthrough,
        InAlphaWord,
    }

    let mut result = String::new();
    let mut word = String::new();
    let mut status = Status::InPassthrough;
    let mut can_start_word = true;
    for c in text.chars().collect::<Vec<char>>() {

        if c.is_alphabetic() {
            match status {
                Status::InPassthrough => {
                    if can_start_word {
                        println!("Pass though: {c}, now in alpha word");
                        word.push(c);
                        status = Status::InAlphaWord;
                    }
                    else {
                        println!("Pass though: {c}, cannot start word");
                        result.push(c);
                    }
                },
                Status::InAlphaWord => {
                    println!("Pass though: {c}, still in alpha word");
                    word.push(c);
                },
            }
        }
        else {
            if c.is_whitespace() || [',', '.', ';', '?', ':', '!'].contains(&c) {
                can_start_word = c.is_whitespace();
                println!("Saw a space, can make new translateable words");
                match status {
                    Status::InPassthrough => {
                        println!("Pass though: {c} - space or standard punctuation");
                        result.push(c);
                    },
                    Status::InAlphaWord => {
                        // terminates word
                        // process the word, push c, change status
                        result += &translate_word(&word);
                        word.clear();
                        result.push(c);
                        status = Status::InPassthrough;
                        println!("Space or punct \"{c}\" terminates word {word}, result: {result}, passthrough {c}");
                    },
                }
            }
            else {
                // Anything weird prevents starting a translateable word until
                // there is a space
                can_start_word = false;
                println!("Saw {c}, disabling alpha words until we see a space");
                match status {
                    Status::InPassthrough => {
                        result.push(c);
                        println!("Pass though non-space, non-alpha: {c}");
                    },
                    Status::InAlphaWord => {
                        // terminates word without translating
                        println!("Abandon word, pass through {word}{c}, now in passthrough");
                        result += &word;
                        word.clear();
                        result.push(c);
                        status = Status::InPassthrough;
                    },
                }
            }
        }
    }

    // End of character processing
    // See if a word is still available
    match status {
        Status::InAlphaWord => {
            // terminates word without translating
            println!("Word remaining: {word}");
            result += &translate_word(&word);
        },
        _ => {},
    }

    result
}

fn translate_word(word: &str) -> String {
    println!("word: {word}");
    let mut result = String::new();
    let characters = word.chars().collect::<Vec<char>>();
    let mut suffix = String::from("");

    for (i, c_ref) in characters.iter().enumerate() {
        println!("{c_ref} is vowel: {}", c_ref.is_romance_vowel());
        if i == 0 {
            
            if c_ref.is_romance_vowel() {
                suffix += "-hay";
                // Push whole word
                result.push(*c_ref);
            }
            else {
                suffix.push('-');
                suffix.push(*c_ref);
                suffix += "ay";
                // Do not push 1st char
            }
        }
        else {
            // Push remaining chars
            result.push(*c_ref);
            println!("pig latin word so far: {}", result);
        }
    }

    // Suffix
    result += &suffix;

    result
}
