use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod character_window {

    #[derive(Debug)]
    pub struct Window {
        vector: [Option<char>; 14],
        char_count: u64
    }

    impl Window {
        pub fn new() -> Window {
            Window { vector: [None; 14], char_count: 0 }
        }

        pub fn push(&mut self, character: char) {
            for i in 0..14 {
                if self.vector[i] == None {
                    self.vector[i] = Some(character);
                    self.char_count += 1;
                    return;
                }
            }

            for i in 0..13 {
                self.vector[i] = self.vector[i+1];
            }
            self.vector[13] = Some(character);
            self.char_count += 1 ;
        }

        pub fn all_chars_unique(&self) -> bool {
            for i in 0..14 {
                if self.vector[i] == None {
                    return false;
                }
            }

            for i in 0..=13 {
                for j in i+1..=13 {
                    if self.vector[i].unwrap() == self.vector[j].unwrap() {
                        return false;
                    }
                }
            }
            return true;
        }

        pub fn get_char_count(&self) -> u64 {
            self.char_count
        }
    }
}

pub fn part1() {
    // Create a path to the desired file
    let path = Path::new("./Resources/Day6.txt");
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

    let mut window = character_window::Window::new();
    let mut marker = 0;

    for character in s.chars() {
        window.push(character);

        if window.all_chars_unique() {
            marker = window.get_char_count();
            break;
        }
    }

    println!("First marker after character {}", marker);
}