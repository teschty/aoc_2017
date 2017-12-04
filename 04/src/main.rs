use std::io::Read;
use std::fs::File;
use std::collections::HashSet;
use std::iter::{Iterator, FromIterator};

// reads in puzzle.txt
fn read_file() -> String {
    let mut f = File::open("puzzle.txt")
        .expect("puzzle.txt couldn't be found :(");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Couldn't read puzzle.txt :(");

    contents
}

fn sort_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));

    String::from_iter(chars)
}

fn is_valid_passphrase(line: &str, map_function: &Fn(&str) -> String) -> bool {
    let mut set = HashSet::new();
    for word in line.split_whitespace().map(map_function) {
        if set.contains(&word) { 
            return false;
        }

        set.insert(word);
    }

    true
}

fn part_one(input: &str) -> i32 {
    // stupid way I made work for fun
    input.lines()
         .map(|line| is_valid_passphrase(line, &|s| s.into()))
         .filter(|valid| *valid)
         .count() as i32
}

fn part_two(input: &str) -> i32 {
    input.lines()
         .map(|line| is_valid_passphrase(line, &sort_string))
         .filter(|valid| *valid)
         .count() as i32
}

fn main() {
    let input = read_file();
    let input = input.trim();
    
    let part_one_solution = part_one(input);
    let part_two_solution = part_two(input);

    println!("Part one solution is {}", part_one_solution);
    println!("Part two solution is {}", part_two_solution);
}
