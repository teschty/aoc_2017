use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashSet;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

enum Move {
    Spin(i32),
    Exchange(usize, usize),
    Partner(char, char)
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = &s[0..1];
        let rest = &s[1..];

        match op {
            "s" => {
                let spin_amount: i32 = rest.parse().unwrap();

                Ok(Move::Spin(spin_amount))
            }
            "x" => {
                let parts: Vec<&str> = rest.split('/').collect();
                let mut first: usize = parts[0].parse().unwrap();
                let mut second: usize = parts[1].parse().unwrap();

                Ok(Move::Exchange(first, second))
            }
            "p" => {
                let first = rest.chars().nth(0).unwrap();
                let second = rest.chars().nth(2).unwrap();

                Ok(Move::Partner(first, second))
            }
            _ => Err(())
        }
    }
}

fn get_string(programs: &[char], start_idx: &i32) -> String {
    let mut rearranged: Vec<char> = vec![];
    let start_idx = *start_idx as usize;
    rearranged.extend(&programs[start_idx..]);
    rearranged.extend(&programs[0..start_idx]);

    rearranged.iter().collect()
}

fn do_dance(programs: &mut [char], dance: &[Move], start_idx: &mut i32) {
    let mut si = *start_idx;

    for cmd in dance {
        use Move::*;

        match *cmd {
            Spin(spin_amount) => {
                si = (si - spin_amount + programs.len() as i32) % programs.len() as i32;
            }

            Exchange(first, second) => {
                let first = (first + si as usize) % programs.len() as usize;
                let second = (second + si as usize) % programs.len() as usize;
                
                programs.swap(first, second);
            }

            Partner(first, second) => {
                let first_idx = programs.iter().position(|&c| c == first).unwrap();
                let second_idx = programs.iter().position(|&c| c == second).unwrap();

                programs.swap(first_idx, second_idx);
            }
        }
    }

    *start_idx = si;
}

fn part_one_and_two(input: &str, num_chars: u8) -> (String, String) {
    let dance: Vec<Move> = input.split(',').filter_map(|s| Move::from_str(s).ok()).collect();
    let mut programs: Vec<char> = (0..num_chars).map(|i| (i + 97) as char).collect();
    let mut start_idx = 0;

    let mut seen = HashSet::new();
    do_dance(&mut programs, &dance, &mut start_idx);
    let first = get_string(&programs, &start_idx);
    seen.insert(programs.clone());

    // find cycle length
    let mut cycle_len = 0;
    loop {
        do_dance(&mut programs, &dance, &mut start_idx);

        if seen.contains(&programs) {
            break; // found it
        }

        seen.insert(programs.clone());
        cycle_len += 1;
    }

    for _ in 0..(1_000_000_000 % cycle_len - 2) {
        do_dance(&mut programs, &dance, &mut start_idx)
    }

    let last = get_string(&programs, &start_idx);
    (first, last)
}


fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let (part_one_solution, part_two_solution) = part_one_and_two(puzzle, 16);
    println!("Solution to part one is {}", part_one_solution);
    println!("Solution to part two is {}", part_two_solution);
}
