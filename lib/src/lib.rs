use core::panic;
use std::{fs::File, path::Path, io::BufReader};

pub fn read_file(filepath: &str) -> BufReader<File> {
    let path = Path::new(filepath);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    return BufReader::new(file);
}