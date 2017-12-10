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

    let mut list = [0i32; LIST_SIZE];

    // initialize with 0..LIST_SIZE
    for i in 0..LIST_SIZE {
        list[i] = i as i32;
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

    list[0] * list[1]
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
}
