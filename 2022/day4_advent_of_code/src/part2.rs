use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn part2() {
    // Create a path to the desired file
    let path = Path::new("./Resources/Day4.txt");
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

    let mut current_val: i64 = 0;

    for string in s.lines() {
        let str: Vec<&str> = string.split(",").collect();

        let elf_1_section_str: Vec<&str> = str[0].split("-").collect();
        let elf_2_section_str: Vec<&str> = str[1].split("-").collect();

        let x: u32 = elf_1_section_str[0].parse::<u32>().unwrap();
        let y: u32 = elf_1_section_str[1].parse::<u32>().unwrap();
        let a: u32 = elf_2_section_str[0].parse::<u32>().unwrap();
        let b: u32 = elf_2_section_str[1].parse::<u32>().unwrap();

        if a <= x && x <= b {
            current_val += 1;
            continue;
        }
        if a <= y && y <= b {
            current_val += 1;
            continue;
        }
        if x <= a && a <= y {
            current_val += 1;
            continue;
        }
        if x <= b && b <= y {
            current_val += 1;
            continue;
        }
    }

    println!("The sum of the priorities is {}", current_val);
}
