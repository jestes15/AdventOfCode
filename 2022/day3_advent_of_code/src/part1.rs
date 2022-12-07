use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

macro_rules! add_to_char {
    ($a:expr, $b:expr) => {
        (($a as u8) + $b) as char
    };
}

pub fn part1() {
    // Create a path to the desired file
    let path = Path::new("./Resources/Day3.txt");
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
        let len = string.chars().count();
        let (substr_1, substr_2) = string.split_at((len / 2) as usize);

        let mut hashmap_1: HashMap<char, u32> = HashMap::new();
        let mut hashmap_2: HashMap<char, u32> = HashMap::new();

        for char in substr_1.chars() {
            hashmap_1.insert( char, if hashmap_1.contains_key(&char) { 1 + hashmap_1[&char] } else { 1 } );
        }

        for char in substr_2.chars() {
            hashmap_2.insert( char, if hashmap_2.contains_key(&char) { 1 + hashmap_2[&char] } else { 1 } );
        }

        for i in 1..=52 {
            if hashmap_1.contains_key(&add_to_char!( 'A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 } )) &&
               hashmap_2.contains_key(&add_to_char!( 'A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 } )) {
                current_val += if &add_to_char!('A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 }) >= &'A' &&
                                  &add_to_char!('A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 }) <= &'Z' { i + 26 } else { i - 26 } as i64;
            }
        }
    }

    println!("The sum of the priorities is {}", current_val);
}
