use std::fs::File;
use std::io::Read;

// reads in puzzle.txt
fn read_file() -> String {
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

// convert ascii number to it's actual value
fn to_number(byte: u8) -> i32 {
    i32::from(byte) - ('0' as i32)
}

fn part_one(bytes: &[u8]) -> i32 {
    let mut sum = 0i32;
    let len = bytes.len();
    for i in 0..len {
        if bytes[i] == bytes[(i + 1) % len] {
            sum += to_number(bytes[i]);
        }
    }

    sum
}

fn part_two(bytes: &[u8]) -> i32 {
    let mut sum = 0i32;
    let len = bytes.len();
    for i in 0..bytes.len() { 
        let mut match_idx = i + bytes.len() / 2;

        if bytes[i] == bytes[match_idx % len] {
            sum += to_number(bytes[i]);
        }
    }

    sum
}

fn main() {
    let puzzle = read_file();
    // trim to remove newline
    let puzzle = puzzle.trim();
    
    // since these are just all ASCII numbers (0-9)
    // we can just convert this to byte array
    let bytes = puzzle.as_bytes();
    let part_one_solution = part_one(bytes);
    let part_two_solution = part_two(bytes);

    println!("Part one solution is {}", part_one_solution);
    println!("Part two solution is {}", part_two_solution);
}
