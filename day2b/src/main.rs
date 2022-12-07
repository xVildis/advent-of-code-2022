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
        if let Some((enemy, output)) = line.split_once(' ') {

            println!("enemy {} you {}", enemy, output);

            let enemy_hand = match enemy {
                "A" => RPS::R,
                "B" => RPS::P,
                "C" => RPS::S,
                _ => unreachable!("invalid enemy hand")
            };

            let mut round_points = 0;
            match output {
                "X" => {
                    round_points += match enemy_hand {
                        RPS::R => 3,
                        RPS::P => 1,
                        RPS::S => 2,
                    }
                }
                "Y" => {
                    round_points += enemy_hand as i32 + 3;
                }
                "Z" => {
                    round_points += match enemy_hand {
                        RPS::R => 2,
                        RPS::P => 3,
                        RPS::S => 1,
                    };
                    round_points += 6;
                }
                _ => unreachable!("invalid output")
            };

            total_points += round_points;
        }
    }
    println!("total points: {total_points}");
}
