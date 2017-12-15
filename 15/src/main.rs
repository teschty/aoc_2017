use std::fs::File;
use std::io::Read;
use std::collections::LinkedList;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: &str) -> u64 {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut gen_a_value: u64 = parts[4].parse().unwrap();
    let mut gen_b_value: u64 = parts[9].parse().unwrap();
    let mut times_matched = 0;

    for _ in 0..40_000_000 {
        gen_a_value = (gen_a_value * 16807) % 2147483647;
        gen_b_value = (gen_b_value * 48271) % 2147483647;

        if gen_a_value & 0xffff == gen_b_value & 0xffff {
            times_matched += 1;
        }
    }

    times_matched
}

fn part_two(input: &str) -> u64 {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut gen_a_value: u64 = parts[4].parse().unwrap();
    let mut gen_b_value: u64 = parts[9].parse().unwrap();
    let mut times_matched = 0;
    let mut values_compared = 0;

    let mut compare_a = LinkedList::new();
    let mut compare_b = LinkedList::new();

    while values_compared < 5_000_000 {
        gen_a_value = (gen_a_value * 16807) % 2147483647;
        gen_b_value = (gen_b_value * 48271) % 2147483647;

        if gen_a_value % 4 == 0 {
            compare_a.push_back(gen_a_value);
        }

        if gen_b_value % 8 == 0 {
            compare_b.push_back(gen_b_value);
        }

        if compare_a.len() > 0 && compare_b.len() > 0 {
            let (val_a, val_b) = (compare_a.pop_front().unwrap(), compare_b.pop_front().unwrap());

            if val_a & 0xffff == val_b & 0xffff {
                times_matched += 1;
            }

            values_compared += 1;
        }
    }

    times_matched
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
    println!("Solution to part two is {}", part_two(puzzle));
}
