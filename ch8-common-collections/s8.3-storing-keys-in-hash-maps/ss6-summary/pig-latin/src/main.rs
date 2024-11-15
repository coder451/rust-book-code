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
        "Be a syzygy."
    ];

    for case in cases {
        println!("\n\nCase: {case}");
        let result = translate(case, false, false);
        println!("Result: {}", result);
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
//   1st char is not vowel-like: move the initial consonant cluster to 
//     the end and add ay.
// Also
//    Preserve an initial capital
//    Treat qu as an initial consonant cluster
fn translate(text: &str, do_debug_print: bool, debug_translate: bool) -> String {
    #[derive(Debug)]
    enum Status {
        InPassthrough,
        InAlphaWord,
    }

    let mut result = String::new();
    let mut word = String::new();
    let mut status = Status::InPassthrough;
    let mut can_start_word = true;
    for c in text.chars().collect::<Vec<char>>() {
        if do_debug_print {
            println!("translate: character: {c}");
        }
        if c.is_alphabetic() {
            if do_debug_print {
                println!("translate: character is alpha");
            }
            match status {
                Status::InPassthrough => {
                    if do_debug_print {
                        println!("translate: status: InPassthrough");
                    }
                    if can_start_word {
                        if do_debug_print {
                            println!("translate: can_start_word");
                        }
                        word.push(c);
                        status = Status::InAlphaWord;
                        if do_debug_print {
                            println!("translate: status -> InAlphaWord; add char to word");
                        }
                    }
                    else {
                        if do_debug_print {
                            println!("translate: add char to result");
                        }
                        result.push(c);
                    }
                },
                Status::InAlphaWord => {
                    if do_debug_print {
                        println!("translate: status: InAlphaWord");
                    }
                    if do_debug_print {
                        println!("translate: add char to word");
                    }
                    word.push(c);
                },
            }
        }
        else {
            if do_debug_print {
                println!("translate: character is non-alpha");
            }
            if c.is_whitespace() || [',', '.', ';', '?', ':', '!'].contains(&c) {
                can_start_word = c.is_whitespace();
                if do_debug_print {
                    println!("translate: character is space or standard punctuation");
                }
                if do_debug_print {
                    println!("translate: can_start_word -> {can_start_word}");
                }
                match status {
                    Status::InPassthrough => {
                        if do_debug_print {
                            println!("translate: status: InPassthrough: add char to result");
                        }
                        result.push(c);
                    },
                    Status::InAlphaWord => {
                        // terminates word
                        // process the word, push c, change status
                        result += &translate_word(&word, debug_translate);
                        word.clear();
                        result.push(c);
                        status = Status::InPassthrough;
                        if do_debug_print {
                            println!("translate: status: InAlphaWord: Space or punct terminates word {word}; translate and add to result, add char to result");
                        }
                    },
                }
            }
            else {
                // Anything weird prevents starting a translateable word until
                // there is a space
                if do_debug_print {
                    println!("translate: character is non-alpha, not space or punctuation");
                }
                can_start_word = false;
                if do_debug_print {
                    println!("translate: can_start_word -> {can_start_word}");
                }
                match status {
                    Status::InPassthrough => {
                        if do_debug_print {
                            println!("translate: status: InPassthrough; add char to result");
                        }
                        result.push(c);
                    },
                    Status::InAlphaWord => {
                        // terminates word without translating
                        if do_debug_print {
                            println!("translate: status: InAlphaWord: Abandon word, add to result, add char to result; status -> InPassthrough");
                        }
                        result += &word;
                        word.clear();
                        result.push(c);
                        status = Status::InPassthrough;
                    },
                }
            }
        }
    }
    if do_debug_print {
        println!("translate: all characters have been processed; look for remaining untranslated word");
    }

    // End of character processing
    // See if a word is still available
    match status {
        Status::InAlphaWord => {
            // terminates word without translating
            if do_debug_print {
                println!("translate: status: InAlphaWord: Word remaining: {word}; translate and add to result");
            }
            result += &translate_word(&word, debug_translate);
        },
        _ => {
            if do_debug_print {
                println!("translate: status: {:?}: no word untranslated", status);
            }
        },
    }

    result
}

/// Find where the initial consonants end, then split the word
fn translate_word(word: &str, debug_translate: bool) -> String {
    if debug_translate {
        println!("translate_word: word: {word}");
    }
    let characters = word.chars().collect::<Vec<char>>();

    let mut head: Vec<char> = vec![];
    let mut tail: Vec<char> = vec![];
    use std::collections::HashSet;
    let extra_vowels: HashSet<char> = "yY".chars().collect();
    let mut in_head = true;
    for c in characters {
        if debug_translate {
            println!("translate_word: char is {c}");
        }        
        if in_head && c.is_romance_vowel_including(&extra_vowels) {
            if debug_translate {
                println!("translate_word: vowel terminates head");
            }
            in_head = false;
            if head.len() > 0 && head[0].to_ascii_lowercase() == 'q' && c == 'u' {
                if debug_translate {
                    println!("translate_word: got qu");
                }
                head.push(c);
                continue;
            }
        }

        if in_head {
            if debug_translate {
                println!("translate_word: add char to head");
            }        
            head.push(c);
        }
        else {
            if debug_translate {
                println!("translate_word: add char to tail");
            }        
            tail.push(c);
        }
    }

    if head.len() > 0 {
        if head[0].is_uppercase() {
            if debug_translate {
                println!("translate_word: leading uppercase adjustment");
            }        
            head[0] = head[0].to_ascii_lowercase();
            if tail.len() > 0 {
                tail[0] = tail[0].to_ascii_uppercase();
            }
        }
    }

    let head: String = head.into_iter().collect();
    let tail: String = tail.into_iter().collect();
    let mut result = String::new();
    result += &tail;
    if head.len() > 0 {
        result += &head;
        result += "ay";
    }
    else {
        result += "fay";
    }
    if debug_translate {
        println!("translate_word: result: {result}");
    }        

    result

}

