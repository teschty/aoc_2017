#![feature(inclusive_range_syntax)] 
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

const LIST_SIZE: usize = 256;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

// modified from day 10 part 2 
fn get_hash(input: &str) -> Vec<u8> {
    let mut lengths: Vec<usize> = input
        .as_bytes().iter()
        .map(|b| usize::from(*b))
        .collect();

    lengths.extend(&[17, 31, 73, 47, 23]);

    let mut list: Vec<u8> = (0..=255).collect();
    let mut current_pos = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in &lengths {
            for i in 0..(length / 2) {
                let start_idx = (current_pos + i) % LIST_SIZE;
                let end_idx = (current_pos + (length - 1) - i) % LIST_SIZE;

                list.swap(start_idx, end_idx);
            }

            current_pos += length + skip_size;
            skip_size += 1;
        }
    }

    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &val| acc ^ val as u8))
        .collect()
}

fn part_one(input: &str) -> u32 {
    (0..128).map(|i| {
        let hash_bytes = get_hash(&format!("{}-{}", input, i));
        hash_bytes.iter().map(|b| b.count_ones()).sum::<u32>()
    }).sum()
}

fn dfs_visit(x: i32, y: i32, grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>) {
    if x < 0 || y < 0 || x > 127 || y > 127 { return }
    let (x, y) = (x as usize, y as usize);
    if visited.contains(&(x, y)) { return }

    if grid[x][y] == '1' {
        visited.insert((x, y));

        // convert back to prevent underflow
        let (x, y) = (x as i32, y as i32);
        dfs_visit(x - 1, y, grid, visited);
        dfs_visit(x + 1, y, grid, visited);
        dfs_visit(x, y - 1, grid, visited);
        dfs_visit(x, y + 1, grid, visited);
    }
}

// this one won't be as easy...
fn part_two(input: &str) -> u32 {
    // convert to 2d 'array' of '0's and '1's
    let grid: Vec<Vec<char>> = (0..128).map(|i| {
        let hash_bytes = get_hash(&format!("{}-{}", input, i));
        let bit_string: String = hash_bytes.iter().map(|b| format!("{:08b}", b)).collect();
        bit_string.chars().collect()
    }).collect();

    let mut regions = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..128 {
        for x in 0..128 {
            if grid[x][y] != '1' {
                continue;
            }

            if !visited.contains(&(x, y)) {
                regions += 1;
                dfs_visit(x as i32, y as i32, &grid, &mut visited);
            }
        }
    }

    regions
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
    println!("Solution to part two is {}", part_two(puzzle));
}
