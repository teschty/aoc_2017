use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: &str) -> i32 {
    let mut severity = 0;

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split(": ")
            .filter_map(|s| s.parse().ok())
            .collect();
        
        let (depth, range) = (parts[0], parts[1]);
        if depth % (range * 2 - 2) == 0 {
            severity += depth * range; 
        }
    }

    severity
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
}
