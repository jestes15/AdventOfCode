use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const TEST: bool = false;

mod queue {
    #[derive(Debug)]
    pub struct QueueData {
        data: Vec<char>,
    }

    impl QueueData {
        pub fn new(vector: Vec<char>) -> QueueData {
            QueueData { data: vector }
        }
        pub fn pop(&mut self) -> char {
            self.data.pop().unwrap()
        }
        pub fn push(&mut self, x: char) {
            self.data.push(x);
        }
        pub fn get_top(&mut self) -> Option<char> {
            match self.data.len() {
                0 => None,
                n => Some(self.data[n-1])
            }
        }
    }
}

pub fn part2() {
    // Create a path to the desired file
    let path: &Path;

    if !TEST {
        path = Path::new("./Resources/Day5.txt");
    } else {
        path = Path::new("./Resources/test.txt");
    }

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

    let mut queue: Vec<queue::QueueData>;

    if !TEST {
        queue = Vec::with_capacity(9);
        queue.push(queue::QueueData::new(Vec::<char>::from(['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['N', 'V', 'G', 'P', 'H', 'W', 'B'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['F', 'W', 'B', 'J', 'G'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['G', 'J', 'N', 'F', 'L', 'W', 'C', 'S'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['W', 'J', 'L', 'T', 'P', 'M', 'S', 'H'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['B', 'C', 'W', 'G', 'F', 'S'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['H', 'T', 'P', 'M', 'Q', 'B', 'W'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['F', 'S', 'W', 'T'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['N', 'C', 'R'])));
    }
    else {
        queue = Vec::with_capacity(3);
        queue.push(queue::QueueData::new(Vec::<char>::from(['Z', 'N'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['M', 'C', 'D'])));
        queue.push(queue::QueueData::new(Vec::<char>::from(['P'])));
    }

    let mut tmp_queue: queue::QueueData = queue::QueueData::new(Vec::<char>::from([]));


    for line in s.lines(){
        let words: Vec<&str> = line.split_whitespace().collect();

        let move_amnt: u32 = words[1].parse::<u32>().unwrap();
        let src: usize = words[3].parse::<usize>().unwrap();
        let dest: usize = words[5].parse::<usize>().unwrap();

        for _ in 0..move_amnt {
            let char = queue[src - 1].pop();
            tmp_queue.push(char);
        }
        for _ in 0..move_amnt {
            let char = tmp_queue.pop();
            queue[dest - 1].push(char);
        }
    }

    for mut i in queue {
        print!("{}", i.get_top().unwrap());
    }
    println!();
}

