use std::io::BufRead;
use lib::read_file;
fn main() {
    star1();
    star2();
}

fn star1() {
    let mut total = 0; 

    for line in read_file("./src/input.txt").lines() {
        if let Ok(l) = line {
            match l.as_str() {
                "A X" => total += 1 + 3,
                "B X" => total += 1 + 0,
                "C X" => total += 1 + 6,
                "A Y" => total += 2 + 6,
                "B Y" => total += 2 + 3,
                "C Y" => total += 2 + 0,
                "A Z" => total += 3 + 0,
                "B Z" => total += 3 + 6,
                "C Z" => total += 3 + 3,
                _ => (),
            };
        }
    }
    println!("Total Score (star1): {}", total);
}

fn star2() {
    let mut total = 0; 

    for line in read_file("./src/input.txt").lines() {
        if let Ok(l) = line {
            match l.as_str() {
                "A X" => total += 3 + 0,
                "B X" => total += 1 + 0,
                "C X" => total += 2 + 0,
                "A Y" => total += 1 + 3,
                "B Y" => total += 2 + 3,
                "C Y" => total += 3 + 3,
                "A Z" => total += 2 + 6,
                "B Z" => total += 3 + 6,
                "C Z" => total += 1 + 6,
                _ => (),
            };
        }
    }
    println!("Total Score (star2): {}", total);
}