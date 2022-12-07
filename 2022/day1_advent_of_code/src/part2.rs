use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn part2() {
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

    let split_string = s.lines();

    let mut elf = 0;
    let mut arr = vec![0];

    for string in split_string {
        if string == "" {
            arr.resize(arr.len() + 1, elf);
            elf = 0;
        }
        else  {
            match string.parse::<i64>() {
                Ok(n) => elf += n,
                Err(why) => println!("{why}")
            }
        }
    }

    arr.sort_by(|a, b| b.cmp(a));

    println!("The top 3 are {0}", arr[0] + arr[1] + arr[2]);
}