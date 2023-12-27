fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);
    let word2 = last_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}, second word = {}", word, word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..bytes.len()];
        }
    }

    &s[..]
}