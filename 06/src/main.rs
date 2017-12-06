use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: &str) -> i32 {
    let mut num_redists = 0;
    let mut set = HashSet::new();

    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    while !set.contains(&numbers) {
        // have to clone, don't think it should be necessary though :(
        set.insert(numbers.clone());

        // couldn't use built in max
        // because of tie issues
        let mut max_idx = 0;
        let mut max_val = i32::min_value();

        for (idx, &num) in numbers.iter().enumerate() {
            if num > max_val {
                max_idx = idx;
                max_val = num;                
            }
        }

        let (mut idx, mut blocks) = (max_idx, max_val);

        // zero out max block
        numbers[idx] = 0;

        // redistribute
        for _ in 0..blocks {
            idx += 1;
            if idx >= numbers.len() { idx = 0; }

            numbers[idx] += 1;
        }

        num_redists += 1;
    }


    num_redists
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let part_one_solution = part_one(&puzzle);
    // let part_two_solution = part_two(&puzzle);

    println!("Solution to part one is {}", part_one_solution);
    // println!("Solution to part two is {}", part_two_solution);
}
