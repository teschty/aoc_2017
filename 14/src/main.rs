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

// modified from day 10 part 2 
fn get_hash(input: &str) -> Vec<u8> {
    let mut lengths: Vec<usize> = input
        .as_bytes().iter()
        .map(|b| usize::from(*b))
        .collect();

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

    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &val| acc ^ val as u8))
        .collect()
}

fn part_one(input: &str) -> u32 {
    (0..128).map(|i| {
        let hash_bytes = get_hash(&format!("{}-{}", input, i));
        hash_bytes.iter().map(|b| b.count_ones()).sum::<u32>()
    }).sum()
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
}
