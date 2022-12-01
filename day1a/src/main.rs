use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    // waits for input
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => {
            println!("{n}");
            //println!("{buffer}");
        }
        Err(error) => println!("{error}")
    }

    let lines = buffer.lines();

    let mut highest_calorie_count: i32 = 0;
    let mut current_calorie_count: i32 = 0;
    for line in lines {
        if line.is_empty() {
            if current_calorie_count > highest_calorie_count {
                highest_calorie_count = current_calorie_count;
            }
            current_calorie_count = 0;
            continue;
        }
        current_calorie_count += line.parse::<i32>().unwrap();
    }

    println!("highest calorie count is: {highest_calorie_count}");

}
