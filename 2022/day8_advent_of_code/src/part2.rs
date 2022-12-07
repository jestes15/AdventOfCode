use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn part2() {
    // Create a path to the desired file
    let path = Path::new("./Resources/Day8.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }
}
