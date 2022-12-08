use std::{io::{self, Read}};

fn main() {
    let mut buffer: String = String::new();
    let mut stdin = io::stdin();
    // waits for input
    match stdin.read_to_string(&mut buffer) {
        Ok(_n) => println!(""),
        Err(error) => println!("{error}")
    }

    let lines = buffer.lines();
    let mut duplicate_ranges:i32 = 0;
    for line in lines {
        let (elf1, elf2) = line.split_once(",").unwrap();
        
        let (elf1_start, elf1_end) = elf1.split_once("-").unwrap();
        let (elf2_start, elf2_end) = elf2.split_once("-").unwrap();

        let elf1_r_start = elf1_start.parse::<i32>().unwrap();
        let elf1_r_end = elf1_end.parse::<i32>().unwrap();

        let elf2_r_start = elf2_start.parse::<i32>().unwrap();
        let elf2_r_end = elf2_end.parse::<i32>().unwrap();

        if elf1_r_start <= elf2_r_start && elf1_r_end >= elf2_r_end {
            println!("({line}) elf 1 contains elf 2");
            duplicate_ranges += 1;
        }
        else if elf2_r_start <= elf1_r_start && elf2_r_end >= elf1_r_end {
            println!("({line}) elf 2 contains elf 1");
            duplicate_ranges += 1;
        }
    }

    println!("ranges that contain one another {duplicate_ranges}");
}