use std::fs::File;
use std::io::Read;
use std::collections::{HashSet, HashMap};

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn part_one(input: &str) -> Option<&str> {
    let mut has_parents = HashSet::new();
    let mut names = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line 
            .split(|c| c == '(' || c == ')' || c == ' ' || c == ',')
            .collect();

        // keep track of all names so we can check them later
        names.push(parts[0]);

        // this is a bit terrible
        for i in 5..parts.len() {
            if i % 2 == 1 {
                has_parents.insert(parts[i]);
            }
        }
    }

    // whoever ISN'T in the set must not on top of any other programs
    for name in names {
        if !has_parents.contains(name) {
            return Some(name); // so they must be the base
        }
    }

    // Uhhhhhhh, couldn't find anything
    None
}

struct Program<'a> {
    name: &'a str,
    weight: i32,
    parents: Vec<&'a str>
}

fn part_two(input: &str, base: &str) -> Option<i32> {
    let mut programs = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line 
            .split(|c| c == '(' || c == ')' || c == ' ' || c == ',')
            .collect();

        // keep track of all names so we can check them later
        let name = parts[0];
        let weight: i32 = parts[2].parse().unwrap();

        let mut parents = vec![];
        // this is a bit terrible
        for i in 5..parts.len() {
            if i % 2 == 1 {
                parents.push(parts[i]);
            }
        }

        programs.insert(name, Program {
            name,
            weight,
            parents
        });
    }

    // recursively calculates weight of program + programs above it
    fn calculate_weight(name: &str, programs: &HashMap<&str, Program>) -> i32 {
        let prog = &programs[name];

        let mut parents_weight = 0;
        for parent in &prog.parents {
            parents_weight += calculate_weight(parent, &programs); 
        }

        prog.weight + parents_weight
    }

    let mut cur_prog = &programs[base];
    let mut correct_weight = 0;
    loop {
        let mut weight_freqs = HashMap::new();
        let mut weight_programs = HashMap::new();
        for parent in &cur_prog.parents {
            let weight = calculate_weight(parent, &programs);
            *weight_freqs.entry(weight).or_insert(0) += 1;
            weight_programs.insert(weight, parent);
        }

        let keys: Vec<&i32> = weight_freqs.keys().collect();
        // if only one key, we're at a balanced section
        // thus this program must be the wrong weight
        if keys.len() <= 1 {
            let total_weight = calculate_weight(cur_prog.name, &programs);
            // correct weight - weight of all parent programs
            // must be the weight we're supposed to be
            return Some(correct_weight - (total_weight - cur_prog.weight));
        }

        let first = weight_freqs[keys[0]];
        let second = weight_freqs[keys[1]];

        let key = if first > second {
            correct_weight = *keys[0];
            keys[1]
        } else {
            correct_weight = *keys[1];
            keys[0]
        };

        // go to unbalanced parent
        cur_prog = &programs[weight_programs[key]];
    }

    None
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let base = match part_one(puzzle) {
        Some(base) => { println!("Solution to part one is {}", base); base }
        None => { println!("Could not find solution to part one!"); return }
    };

    match part_two(puzzle, base) {
        Some(weight) => println!("Solution to part two is {}", weight),
        None => println!("Could not find solution to part two"),
    };
}
