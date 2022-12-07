use std::{io::{self, Read}};

fn main() {
    let mut buffer: String = String::new();
    let mut stdin = io::stdin();
    // waits for input
    match stdin.read_to_string(&mut buffer) {
        Ok(_n) => println!(""),
        Err(error) => println!("{error}")
    }

    let mut total_priority: u32 = 0;
    
    for i in (0..buffer.lines().count()).step_by(3) {
        let elf1 = buffer.lines().nth(i).unwrap();
        let elf2 = buffer.lines().nth(i + 1).unwrap();
        let elf3 = buffer.lines().nth(i + 2).unwrap();

        for c1 in elf1.chars() {
            if elf2.contains(c1) && elf3.contains(c1) {
                total_priority += match c1 {
                    'a'..='z' => c1 as u32 - 'a' as u32 + 1,
                    'A'..='Z' => c1 as u32 - 'A' as u32 + 27,
                    _ => 0
                };
                break;
            }
        }
    }

    println!("total priority {total_priority}");
}
