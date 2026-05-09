use std::io;

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn main() {
    println!("Enter a string:");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let word = first_word(&s);
    println!("First word: {}", word);
}
