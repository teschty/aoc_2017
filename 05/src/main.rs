use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut f = File::open("puzzle.txt")
        .expect("puzzle.txt couldn't be found :(");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: &str) -> i32 {
    let mut arr: Vec<i32> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut idx = 0i32;
    let mut steps = 0;

    while idx >= 0 && idx < arr.len() as i32 {
        let i = idx as usize;
        let val = arr[i] as i32;

        arr[i] += 1;
        steps += 1;

        idx += val;
    }

    steps
}

fn part_two(input: &str) -> i32 {
    let mut arr: Vec<i32> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut idx = 0i32;
    let mut steps = 0;

    while idx >= 0 && idx < arr.len() as i32 {
        let i = idx as usize;
        let val = arr[i] as i32;

        arr[i] += if val >= 3 { -1 } else { 1 };
        steps += 1;

        idx += val;
    }

    steps
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let part_one_solution = part_one(&puzzle);
    let part_two_solution = part_two(&puzzle);

    println!("Solution to part one is {}", part_one_solution);
    println!("Solution to part two is {}", part_two_solution);
}
