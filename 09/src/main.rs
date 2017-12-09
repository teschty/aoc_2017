use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one_and_two(input: &str) -> (i32, i32) {
    let mut total_score = 0;
    let mut group_level = 0;
    let mut in_garbage = false;
    let mut non_canceled_chars = 0;

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if in_garbage {
            match c {
                '!' => { chars.next(); } // eat next
                '>' => { in_garbage = false; }
                _ => { non_canceled_chars += 1; }
            }

            continue
        }

        match c {
            '{' => { group_level += 1 }
            '}' => {
                total_score += group_level;
                group_level -= 1;
            }
            '<' => { in_garbage = true }
            ',' => {} // don't actually do anything for commas
            _ => { unreachable!() } // not in garbage, shouldn't be able to get here
        }
    }

    (total_score, non_canceled_chars)
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let (part_one_solution, part_two_solution) = part_one_and_two(puzzle);
    println!("Solution to part one is {}", part_one_solution);
    println!("Solution to part two is {}", part_two_solution);
}
