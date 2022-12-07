use std::io::{self, Read};

#[derive(PartialEq)]
enum RPS {
    R = 1,
    P,
    S
}

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

    let mut total_points: i32 = 0;
    for line in buffer.lines() {
        if let Some((enemy, you)) = line.split_once(' ') {
            // handle command, options

            println!("enemy {} you {}", enemy, you);

            let enemy_hand = match enemy {
                "A" => RPS::R,
                "B" => RPS::P,
                "C" => RPS::S,
                _ => unreachable!("invalid enemy hand")
            };

            let your_hand = match you {
                "X" => RPS::R,
                "Y" => RPS::P,
                "Z" => RPS::S,
                _ => unreachable!("invalid hand")
            };

            let mut round_points = your_hand as i32;

            if enemy_hand == your_hand {
                round_points += 3;
            } else if (enemy_hand == RPS::R && your_hand == RPS::P) 
                   || (enemy_hand == RPS::P && your_hand == RPS::S)
                   || (enemy_hand == RPS::S && your_hand == RPS::R) {
                round_points += 6;
            }

            total_points += round_points;
        }
    }
    println!("total points: {total_points}");
}
