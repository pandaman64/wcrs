use std::env;
use std::fs::File;
use std::io::Read;

fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\r' | '\n' => true,
        _ => false,
    }
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path).unwrap(); //assume encoded in utf-8

    let mut word_start = true;
    let mut prev_char = None;

    let mut bytes: u64 = 0;
    // let mut chars = 0;
    let mut words: u64 = 0;
    let mut newlines: u64 = 0;

    let mut buffer = [0; 1024];
    while let Ok(n) = file.read(&mut buffer[..]) {
        if n == 0{
            break;
        }
        for i in 0..n{
            let c = buffer[i] as char;
            bytes += 1;
            // chars += 1;

            if word_start && !is_whitespace(c) {
                words += 1;
                word_start = false;
            } else if is_whitespace(c) {
                word_start = true;
            }

            if c == '\n' && prev_char != Some('\r') {
                newlines += 1;
            } else if c == '\r' {
                newlines += 1;
            }

            prev_char = Some(c);
        }
    }

    println!("{} {} {}", newlines, words, bytes);
}
