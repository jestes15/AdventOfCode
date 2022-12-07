use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn part1() {
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

        let mut elf_1_section: Vec<u32> = vec![0, 0];
        let mut elf_2_section: Vec<u32> = vec![0, 0];

        elf_1_section[0] = elf_1_section_str[0].parse::<u32>().unwrap();
        elf_1_section[1] = elf_1_section_str[1].parse::<u32>().unwrap();
        elf_2_section[0] = elf_2_section_str[0].parse::<u32>().unwrap();
        elf_2_section[1] = elf_2_section_str[1].parse::<u32>().unwrap();

        if elf_1_section[0] <= elf_2_section[0] &&
           elf_1_section[1] >= elf_2_section[1] {
            current_val += 1;
            continue;
        }
        else if elf_2_section[0] <= elf_1_section[0] &&
                elf_2_section[1] >= elf_1_section[1] {
            current_val += 1;
            continue;
        }
    }

    println!("The sum of the priorities is {}", current_val);
}
