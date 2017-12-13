use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn get_layers(input: &str) -> Vec<(i32, i32)> {
    let mut layers = vec![];

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split(": ")
            .filter_map(|s| s.parse().ok())
            .collect();
        
        let (depth, range) = (parts[0], parts[1]);
        layers.push((depth, range));
    }

    layers
}

fn part_one(input: &str) -> i32 {
    let layers = get_layers(input);

    layers.iter()
        .filter(|&&(depth, range)| depth % (range * 2 - 2) == 0)
        .fold(0, |acc, &(depth, range)| acc + (depth * range))
}

fn part_two(input: &str) -> i32 {
    let layers = get_layers(input);
    let mut delay = 0;

    // might be possible to calculate this in a smart way
    // but let's just brute force it
    while layers.iter().any(|&(depth, range)| (depth + delay) % (range * 2 - 2) == 0) {
        delay += 1;
    }

    delay
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
    println!("Solution to part two is {}", part_two(puzzle));
}
