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
    // build adjacency list
    let mut list = vec![];

    for line in input.lines() {
        let adjacent_list = line.split("<->").nth(1).unwrap();
        let adjacent: Vec<usize> = adjacent_list
            .trim()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect(); 
        
        list.push(adjacent);
    }

    // keep track of what we've visited
    let mut visited = HashSet::new();
    // maintain stack of nodes to visit
    let mut to_visit = vec![0];
    // track how many are connected to zero
    let mut num_connected = 0;

    while let Some(node_idx) = to_visit.pop() {
        visited.insert(node_idx);
        num_connected += 1;

        let adjacent_nodes = &list[node_idx];
        for &node in adjacent_nodes {
            if !visited.contains(&node) {
                to_visit.push(node);
            }
        }
    }

    num_connected
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
}