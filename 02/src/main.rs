use std::fs::File;
use std::io::Read;
use std::cmp;

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
    let mut sum = 0;

    for line in input.lines() {
        let numbers: Vec<i32> = line.split('\t').map(|s| s.parse().unwrap()).collect();

        let mut max = i32::min_value();
        let mut min = i32::max_value();

        for num in numbers {
            max = cmp::max(num, max);
            min = cmp::min(num, min);
        }

        sum += max - min;
    }

    sum
}

fn part_two(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let numbers: Vec<i32> = line.split('\t').map(|s| s.parse().unwrap()).collect();

        // there must be a more efficient solution than O(n^2)
        // but I can't think of it at the moment
        'outer: 
        for n in &numbers {
            for m in &numbers {
                if n != m && n % m == 0 {
                    sum += n / m;
                    break 'outer;
                }
            }
        }
    }

    sum
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let part_one_solution = part_one(puzzle);
    let part_two_solution = part_two(puzzle);

    println!("Part one solution is {}", part_one_solution);
    println!("Part two solution is {}", part_two_solution);
}
