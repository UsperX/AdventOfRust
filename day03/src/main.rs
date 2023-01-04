use lib::read_file;
use std::io::BufRead; 

fn main() {
    star1();
    star2(); 
}

fn star1() {
    let mut total = 0; 

    for line in read_file("./src/input.txt").lines() {
        if let Ok(l) = line {
            let (sub1, sub2) = l.split_at(l.len()/2);
            let mut comp2 = sub2.to_string();
            for item in sub1.chars() {
                if comp2.contains(item) {
                    total += item_value(item);
                    comp2 = comp2.replace(item, ""); 
                }
            } 
        }
    }
    println!("Total priority sum: {}", total); 
}

fn star2(){
    let mut total = 0; 
    let mut rucksack1 = "".to_string(); 
    let mut rucksack2 = "".to_string();
    let mut i = 0;  

    for line in read_file("./src/input.txt").lines() {
        if let Ok(l) = line {
            if i == 0 {
                rucksack1 = l; 
                i+=1; 
            } else if i == 1 {
                rucksack2 = l; 
                i+=1; 
            } else {
                for item in l.chars() {
                    if rucksack1.contains(item) && rucksack2.contains(item) {
                        total += item_value(item); 
                        rucksack1 = "".to_string(); 
                        rucksack2 = "".to_string(); 
                        i = 0; 
                        break; 
                    }
                }
            }
        }
    }
    println!("Total badge priority: {}", total);
}

fn item_value(item: char) -> i32 {
    let value = item as i32;
    if value < 97 {
        return value - 38; 
    } else {
        return value - 96;
    }
} 