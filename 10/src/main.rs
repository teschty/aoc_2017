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

    let mut list = [0u8; LIST_SIZE];

    // initialize with 0..LIST_SIZE
    for i in 0..LIST_SIZE {
        list[i] = i as u8;
    }

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

    list[0] as i32 * list[1] as i32
}

fn part_two(input: &str) -> String {
    let mut lengths: Vec<usize> = input
        .as_bytes().iter()
        .map(|b| usize::from(*b))
        .collect();

    // add these values for some reason 
    lengths.extend(&[17, 31, 73, 47, 23]);

    let mut list = [0u8; LIST_SIZE];
    // initialize with 0..LIST_SIZE
    for i in 0..LIST_SIZE {
        list[i] = i as u8;
    }

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

    // make dense hash
    let mut dense_hash = [0u8; LIST_SIZE / 16];
    for i in 0..dense_hash.len() {
        let mut value = 0;
        for j in 0..16 {
            value ^= list[i * 16 + j];
        }

        dense_hash[i] = value;
    }

    let mut hash_string = String::new();
    for byte in &dense_hash {
        hash_string.push_str(&format!("{:x}", byte));
    }

    hash_string
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
    println!("Solution to part two is {}", part_two(puzzle));
}
