use std::{cmp, io::BufRead};
use lib::read_file;
fn main() {
    star1();
    star2(); 
}

fn star1() {
    let mut most_calories = 0; 
    let mut current_elf = 0; 
   
    for line in read_file("./src/input.txt").lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                current_elf = 0; 
            } else {
            current_elf += l.parse::<i32>().unwrap();
            most_calories = cmp::max(current_elf, most_calories);
            }
        }
    }
    println!("Highest total calories: {}", most_calories);
}

fn star2() {
    let mut top3 = [0; 3];
    let mut current_elf = 0; 

    for line in read_file("./src/input.txt").lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                if current_elf > top3[0] {
                    top3[0] = current_elf;
                    top3.sort();
                }
                current_elf = 0; 
            } else {
            current_elf += l.parse::<i32>().unwrap();
            }
        }
    }
    println!("Total of top 3 elves: {}", top3.iter().sum::<i32>());
}