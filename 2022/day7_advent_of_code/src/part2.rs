use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn min(x: u64, y: u64) -> u64 {
    if x > y { y } else { x }
}

pub fn part2() {
    // Create a path to the desired file
    let path = Path::new("./Resources/Day7.txt");
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

    let mut path: Vec<String> = Vec::from([]);
    let mut tree: HashMap<String, u64> = HashMap::new();

    for line in s.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();

        if split_line[1] == "cd" {
            if split_line[2] == ".." {
                path.pop();
            } else {
                path.append(&mut Vec::from([String::from(split_line[2])]));
            }
        } else if split_line[1] == "ls" {
            continue;
        } else if split_line[0] == "dir" {
            continue;
        } else {
            let mut size = 0;
            match split_line[0].parse::<u64>() {
                Ok(n) => {
                    size = n
                }
                Err(e) => {
                    println!("{:?}", split_line);
                    println!("{}", e);
                }
            };
            let mut path_str: String = String::from("");

            for i in 0..path.len() + 1 {

                for i in 0..i {
                    path_str.push_str("/");
                    path_str.push_str(&path[i][..]);
                }

                tree.insert( path_str.clone(), if tree.contains_key(&path_str) { size + tree[&path_str] } else { size } );
            }
        }
    }

    let mut dir_size: u64 = 0xffffffffffff;

    let max_used = 70000000 - 30000000;
    let need_to_free = tree["//"] - max_used;

    for (_, j) in tree {
        if j >= need_to_free {
            dir_size = min(dir_size, j)
        }
    }

    println!("Smallest dir needed to freed {}", dir_size);

}
