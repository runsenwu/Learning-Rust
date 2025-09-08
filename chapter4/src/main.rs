use std::io;

fn main() {
    let mut input = String::new();
    
    let result = io::stdin().read_line(&mut input);

    let ret = match_words(&input);

    println!("The first word of the line is: {ret}");
}

fn match_words(s: &String) -> &str {
    // find the first word
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}