use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: &str) -> Option<&str> {
    let mut has_parents = HashSet::new();
    let mut names = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line 
            .split(|c| c == '(' || c == ')' || c == ' ' || c == ',')
            .collect();

        // keep track of all names so we can check them later
        names.push(parts[0]);

        // this is a bit terrible
        for i in 5..parts.len() {
            if i % 2 == 1 {
                has_parents.insert(parts[i]);
            }
        }
    }

    // whoever ISN'T in the set must not on top of any other programs
    for name in names {
        if !has_parents.contains(name) {
            return Some(name); // so they must be the base
        }
    }

    // Uhhhhhhh, couldn't find anything
    None
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    match part_one(puzzle) {
        Some(base) => println!("Solution to part one {}", base),
        None => println!("Could not find solution to part one!")
    }
}
