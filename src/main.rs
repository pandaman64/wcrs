use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap(); //assume encoded in utf-8

    let mut word_start = true;
    let mut prev_char = None;

    let mut bytes: u64 = 0;
    // let mut chars = 0;
    let mut words: u64 = 0;
    let mut newlines: u64 = 0;

    let mut iter = file.bytes();
    while let Some(Ok(c)) = iter.next() {
        let c = c as char;
        bytes += 1;
        // chars += 1;

        if word_start && !c.is_whitespace() {
            words += 1;
            word_start = false;
        } else if c.is_whitespace() {
            word_start = true;
        }

        if c == '\n' && prev_char != Some('\r') {
            newlines += 1;
        } else if c == '\r' {
            newlines += 1;
        }

        prev_char = Some(c);
    }

    println!("{} {} {}", newlines, words, bytes);
}
