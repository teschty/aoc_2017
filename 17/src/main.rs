use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: usize) -> usize {
    // only 2017 additions,
    // vector will be fine
    let mut buffer = vec![0];
    let mut cur_pos = 0;

    for i in 1..2018 {
        cur_pos = (cur_pos + input) % i;
        buffer.insert(cur_pos + 1, i);
        cur_pos += 1;
    }

    buffer[(cur_pos + 1) % buffer.len()]
}

fn main() {
    let puzzle = read_file();
    let puzzle: usize = puzzle.trim().parse().unwrap();

    println!("Solution to part one is {}", part_one(puzzle));
}
