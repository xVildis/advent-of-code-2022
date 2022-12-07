use std::io::{self, Read};

#[derive(PartialEq)]
enum RPS {
    R = 1,
    P,
    S
}

struct Round {
    enemy: RPS,
    you: RPS
}

impl Round {
    fn new(enemy: &str, you: &str) -> Round {
        Round {
            enemy: match enemy {
                "A" => RPS::R,
                "B" => RPS::P,
                "C" => RPS::S,
                _ => unreachable!("invalid input")
            },
            you: match you {
                "X" => RPS::R,
                "Y" => RPS::P,
                "Z" => RPS::S,
                _ => unreachable!("invalid input")
            }
        }
    }
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
            let round = Round::new(enemy, you);

            println!("enemy {} you {}", round.enemy as i32, round.you as i32);

            let mut round_points = round.you as i32;

            if round.enemy == round.you {
                round_points += 3;
            } else if (round.enemy == RPS::R && round.you == RPS::P) 
                   || (round.enemy == RPS::P && round.you == RPS::S)
                   || (round.enemy == RPS::S && round.you == RPS::R) {
                round_points += 6;
            }

            total_points += round_points;
        } else {
            unreachable!("invalid input");
        }
    }
    println!("total points: {total_points}");
}
