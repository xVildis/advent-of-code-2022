use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    // waits for input
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("{n}"),
        Err(error) => println!("{error}")
    }

    let lines = buffer.lines();

    let mut calorie_counts: Vec<i32> = Vec::new();
    let mut current_calorie_count: i32 = 0;
    for line in lines {
        if line.is_empty() {
            calorie_counts.push(current_calorie_count);
            current_calorie_count = 0;
            continue;
        }
        current_calorie_count += line.parse::<i32>().unwrap();
    }

    calorie_counts.sort();

    println!("top 3 highest calorie counts combined: {}", calorie_counts.iter().rev().take(3).sum::<i32>());

}
