use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one_and_two(input: &str) -> (i32, i32) {
    let mut num_redists = 0;
    let mut map = HashMap::new();

    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let len = numbers.len();
    while !map.contains_key(&numbers) {
        // have to clone, don't think it should be necessary though :(
        map.insert(numbers.clone(), num_redists);

        // couldn't use built in max, because of tie issues
        // so calculate it "manually"
        let (mut max_idx, mut num_blocks) = (0, i32::min_value());

        for (idx, &num) in numbers.iter().enumerate() {
            if num > num_blocks {
                max_idx = idx;
                num_blocks = num;                
            }
        }

        // zero out max block
        numbers[max_idx] = 0;

        // redistribute
        for i in 0..num_blocks {
            numbers[(max_idx + i as usize + 1) % len] += 1;
        }

        num_redists += 1;
    }

    let first_cycle = &map[&numbers.clone()];
    (num_redists, num_redists - first_cycle)
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let (part_one_solution, part_two_solution) = part_one_and_two(puzzle);

    println!("Solution to part one is {}", part_one_solution);
    println!("Solution to part two is {}", part_two_solution);
}
