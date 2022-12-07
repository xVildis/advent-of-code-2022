use std::{io::{self, Read}};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    // waits for input
    match stdin.read_to_string(&mut buffer) {
        Ok(_n) => println!(""),
        Err(error) => println!("{error}")
    }

    let mut total_priority = 0;
    for line in buffer.lines() {
        let (c1, c2) = line.split_at(line.chars().count() / 2);
        
        let mut line_priority = 0;
        for c1_char in c1.chars() {
            for c2_char in c2.chars() {
                if c1_char == c2_char {
                    let duplicate_char = c1_char;
                    line_priority = match duplicate_char {
                        'a'..='z' => duplicate_char as u32 - 'a' as u32 + 1,
                        'A'..='Z' => duplicate_char as u32 - 'A' as u32 + 27,
                        _ => 0
                    };
                }
            }
        }

        total_priority += line_priority;
    }

    println!("priority total: {}", total_priority);
}
