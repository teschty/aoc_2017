#![feature(inclusive_range_syntax)] 

use std::fs::File;
use std::io::Read;

const LIST_SIZE: usize = 256;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: &str) -> i32 {
    let lengths: Vec<usize> = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut list: Vec<u8> = (0..=255).collect();

    let mut current_pos = 0;
    let mut skip_size = 0;

    for length in lengths {
        for i in 0..(length / 2) {
            let start_idx = (current_pos + i) % LIST_SIZE;
            let end_idx = (current_pos + (length - 1) - i) % LIST_SIZE;

            list.swap(start_idx, end_idx);
        }

        current_pos += length + skip_size;
        skip_size += 1;
    }

    i32::from(list[0]) * i32::from(list[1])
}

fn part_two(input: &str) -> String {
    let mut lengths: Vec<usize> = input
        .as_bytes().iter()
        .map(|b| usize::from(*b))
        .collect();

    // add these values for some reason 
    lengths.extend(&[17, 31, 73, 47, 23]);

    let mut list: Vec<u8> = (0..=255).collect();

    let mut current_pos = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in &lengths {
            for i in 0..(length / 2) {
                let start_idx = (current_pos + i) % LIST_SIZE;
                let end_idx = (current_pos + (length - 1) - i) % LIST_SIZE;

                list.swap(start_idx, end_idx);
            }

            current_pos += length + skip_size;
            skip_size += 1;
        }
    }

    let dense_hash: Vec<String> = list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &val| acc ^ val as u8))
        .map(|val| format!("{:x}", val))
        .collect();
    
    dense_hash.join("")
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
    println!("Solution to part two is {}", part_two(puzzle));
}
