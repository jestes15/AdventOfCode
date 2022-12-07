use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn part1() {
    // Create a path to the desired file
    let path = Path::new("./Resources/Day1.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {},
    }

    let mut current_max: i64 = 0;
    let mut elf = 0;
    for string in s.lines() {
        if string == "" {
            if current_max < elf {
                current_max = elf;
            }
            elf = 0;
        }
        else  {
            match string.parse::<i64>() {
                Ok(n) => elf += n,
                Err(why) => println!("{why}")
            }
        }
    }

    println!("The most calories is {current_max}")
}