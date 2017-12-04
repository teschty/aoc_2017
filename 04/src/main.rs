use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

// reads in puzzle.txt
fn read_file() -> String {
    let mut f = File::open("puzzle.txt")
        .expect("puzzle.txt couldn't be found :(");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: &str) -> i32 {
    let mut num_valid = 0;

    'outer: 
    for line in input.lines() {
        let mut set = HashSet::new();

        for word in line.split_whitespace() {
            if set.contains(word) {
                continue 'outer;
            }

            set.insert(word);
        }

        num_valid += 1;
    }

    num_valid
}

fn main() {
    let input = read_file();
    let input = input.trim();
    
    let part_one_solution = part_one(input);

    println!("Part one solution is {}", part_one_solution);
}
