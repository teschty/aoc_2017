use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

#[derive(Copy, Clone)]
enum Direction {
    Up, Down
}

// lol this is bad
fn part_one(input: &str) -> i32 {
    let mut layers = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        
        let layer: i32 = parts[0].parse().unwrap();
        let depth: i32 = parts[1].parse().unwrap();

        layers.push((layer, depth));
    }

    // init to Direction::Up because first loop will reverse
    let mut scanner_states = vec![(0, Direction::Up); layers.len()];
    let mut severity = 0;

    for i in 0..layers.len() {
        if let (0, _) = scanner_states[i] {
            let (depth, range) = layers[i];
            severity += depth * range;
        }

        let dist = if i + 1 < layers.len() {
            layers[i + 1].0 - layers[i].0
        } else {
            0
        };

        // advance scanners as many times as the distance between
        // this layer and the next
        for _ in 0..dist {
            for i in 0..layers.len() {
                let (pos, dir) = scanner_states[i];
                let (_, range) = layers[i];

                // switch directions at ends
                let new_dir = if pos == 0 {
                    Direction::Down
                } else if pos == range - 1 {
                    Direction::Up
                } else {
                    dir
                };

                let new_pos = match new_dir {
                    Direction::Up => pos - 1,
                    Direction::Down => pos + 1
                };

                scanner_states[i] = (new_pos, new_dir);
            }
        }
    }

    severity
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
}
