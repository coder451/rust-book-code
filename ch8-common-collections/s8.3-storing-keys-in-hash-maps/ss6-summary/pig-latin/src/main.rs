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
        let result = translate(case, false, true);
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

fn translate_word(word: &str, debug_translate: bool) -> String {
    if debug_translate {
        println!("translate_word: word: {word}");
    }
    let characters = word.chars().collect::<Vec<char>>();

    let mut cluster = String::new();
    let mut rest  = String::new();
    let mut found_cluster = false;
    let mut uppercase_needed = false;
    use std::collections::HashSet;
    let extra_vowels: HashSet<char> = "yY".chars().collect();
    let mut  is_special = false;
    for c in characters {
        if debug_translate {
            println!("translate_word: char is {c}");
        }

        if ! found_cluster {
            if c.is_romance_vowel_including(&extra_vowels) {
                if debug_translate {
                    println!("translate_word: char is a vowel");
                }
                // Check for any special clusters
                let special = vec!["qu"];
                let mut test_string = cluster.clone();
                test_string.push(c);
                let test_string_lc = test_string.to_ascii_lowercase();
                for sp in special {
                    if debug_translate {
                        println!("translate_word: check for special consonant cluster {sp}");
                    }
                    if sp == test_string_lc {
                        // treat this as a consonant cluster
                        // is_special prevents this char from being added to the rest
                        cluster.push(c);
                        is_special = true;
                        if debug_translate {
                            println!("translate_word: found special consonant cluster; add char to it");
                        }
                        break;
                    }
                }
                // At this point we have found the initial consonant cluster, if any
                found_cluster = true;
                if is_special {
                    is_special = false;
                    if debug_translate {
                        println!("translate_word: finished processing this char; have cluster, need next char")
                    }
                    continue;
                }
                if debug_translate {
                    println!("translate_word: cluster {cluster} is not a special case")
                }
            }
        }

        if found_cluster {
            if debug_translate {
                println!("translate_word: non-special consonant cluster complete");
            }
            if uppercase_needed {
                if debug_translate {
                    println!("translate_word: start of cluster is uppercase, add uppercased char to tail");
                }
                rest.push(c.to_ascii_uppercase());
                uppercase_needed = false;
            }
            else {
                if debug_translate {
                    println!("translate_word: add char to tail");
                }
                rest.push(c);
            }
        }
        else {
            if debug_translate {
                println!("translate_word: consonant cluster not yet complete");
            }
            if cluster.len() == 0 && c.is_uppercase() {
                if debug_translate {
                    println!("translate_word: first char of consonant cluster is uppercase; add lowercase version to cluster");
                }
                cluster.push(c.to_ascii_lowercase());
                uppercase_needed = true;
            }
            else {
                if debug_translate {
                    println!("translate_word: add char to cluster");
                }
                cluster.push(c);
            }
        }
    }
    if debug_translate {
        println!("translate_word: finished processing word characters");
    }

    let mut suffix = String::new();
    if cluster.len() == 0 {
        if debug_translate {
            println!("translate_word: word starts with vowel");
        }
        suffix += "f";
    }
    else {
        if debug_translate {
            println!("translate_word: word starts with consonant cluster {cluster}");
        }
        suffix += &cluster;
    }
    suffix += "ay";

    let mut result = String::new();
    result += &rest;
    result += &suffix;
    if debug_translate {
        println!("translate_word: suffix is is {suffix}, result is {result}");
    }
    result
}
