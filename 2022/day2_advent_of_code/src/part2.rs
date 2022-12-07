use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub enum HandResult {
    Win,
    Lose,
    Draw
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub trait  Beats {
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            self::Hand::Rock => self::Hand::Scissors,
            self::Hand::Paper => self::Hand::Rock,
            self::Hand::Scissors => self::Hand::Paper,
        }
    }
}

pub fn play_hand(own_hand: Hand, other_hand: Hand) -> HandResult {
    let (own_beats, other_beats) = (own_hand.beats(), other_hand.beats());

    match (own_beats, other_beats) {
        _ if own_beats == other_hand => self::HandResult::Win,
        _ if other_beats == own_hand => self::HandResult::Lose,
        _                            => self::HandResult::Draw,
    }
}

pub fn get_points(hand: Hand) -> i64 {
    match hand {
        self::Hand::Rock => 1,
        self::Hand::Paper => 2,
        self::Hand::Scissors => 3,
    }
}

pub fn get_result_of_rnd(round: &mut Vec<&str>) -> i64 {
    let cpu_hand = match round[0] {
        "A" => self::Hand::Rock,
        "B" => self::Hand::Paper,
        "C" => self::Hand::Scissors,
        _ => self::Hand::Rock,
    };

    let outcome = match round[1] {
        "X" => self::HandResult::Lose,
        "Y" => self::HandResult::Draw,
        "Z" => self::HandResult::Win,
        _ => self::HandResult::Lose,
    };

    let elf_hand = match outcome {
        _ if play_hand(self::Hand::Rock, cpu_hand) == outcome => self::Hand::Rock,
        _ if play_hand(self::Hand::Paper, cpu_hand) == outcome => self::Hand::Paper,
        _ if play_hand(self::Hand::Scissors, cpu_hand) == outcome => self::Hand::Scissors,
        _ => self::Hand::Rock,
    };

    match outcome {
        self::HandResult::Win => get_points(elf_hand) + 6,
        self::HandResult::Draw => get_points(elf_hand) + 3,
        self::HandResult::Lose => get_points(elf_hand) + 0,
    }
}

pub fn part2() {
    // Create a path to the desired file
    let path = Path::new("./Resources/Day2.txt");
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

    let mut current_points: i64 = 0;
    for string in split_string {
        let mut str_arr: Vec<&str> = string.split(" ").collect();
        current_points += get_result_of_rnd(&mut str_arr);
    }

    println!("Final point value {current_points}")
}