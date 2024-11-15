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
        "this; that",
        "The quick brown fox jumped over the lazy dog.",
        "I think, therefore I am.",
        "Be a syzygy"
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
//   1st char is vowel-like: add fay
//   1st char is not vowel-like: move the initial consonant cluster to the end and add ay.
// Also
//    Preserve an initial capital
//    Treat qu as an initial consonant cluster
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
                        println!("Found: {c}, now in alpha word");
                        word.push(c);
                        status = Status::InAlphaWord;
                    }
                    else {
                        println!("Pass though: {c}, cannot start word");
                        result.push(c);
                    }
                },
                Status::InAlphaWord => {
                    println!("Found: {c}, still in alpha word");
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

    let mut cons = String::new();
    let mut rest  = String::new();
    let mut found_vowel = false;
    let mut case_needed = false;
    use std::collections::HashSet;
    let extra_vowels: HashSet<char> = "yY".chars().collect();
    for c in characters {
        if c.is_romance_vowel_including(&extra_vowels) {
            // Check for any special clusters
            let special = vec!["qu"];
            let mut test_string = cons.clone();
            test_string.push(c);
            let test_string_lc = test_string.to_ascii_lowercase();
            for sp in special {
                if sp == test_string_lc {
                    // treat this as a consonant cluster
                    cons.push(c);
                    break;
                }
            }
            // At this point we have found the initial consonant cluster, if any
            found_vowel = true;
        }

        if found_vowel {
            if case_needed {
                rest.push(c.to_ascii_lowercase());
                case_needed = false;
            }
            else {
                rest.push(c);
            }
        }
        else {
            if cons.len() == 0 && c.is_uppercase() {
                cons.push(c.to_ascii_lowercase());
                case_needed = true;
            }
            else {
                cons.push(c);
            }
        }
    }

    if cons.len() == 0 {
        cons += "f";
    }

    result += &rest;
    result += &cons;
    result += "ay";
    result
}
