use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use itertools::Itertools;

macro_rules! add_to_char {
    ($a:expr, $b:expr) => {
        (($a as u8) + $b) as char
    };
}

pub fn part2() {
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

    for string_chunk in &s.lines().chunks(3) {

        let mut hashmap_array: [HashMap<char, u32>; 3] = [HashMap::new(), HashMap::new(), HashMap::new()];

        for (i, line) in string_chunk.enumerate() {
            for char in line.chars() {
                hashmap_array[i].insert(char, if hashmap_array[i].contains_key(&char) { 1 + hashmap_array[i][&char] } else { 1 });
            }
        }

        for i in 1..=52 {
            if hashmap_array[0].contains_key(&add_to_char!( 'A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 } )) &&
               hashmap_array[1].contains_key(&add_to_char!( 'A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 } )) &&
               hashmap_array[2].contains_key(&add_to_char!( 'A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 } )) {
                current_val += if &add_to_char!('A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 }) >= &'A' &&
                                  &add_to_char!('A', if (i - 1) >= 26 { i - 1 + 6 } else { i - 1 }) <= &'Z' { i + 26 } else { i - 26 } as i64;
            }
        }
    }

    println!("The sum of the priorities is {}", current_val);
}
