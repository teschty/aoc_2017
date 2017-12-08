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
    let mut registers = HashMap::new();
    let mut max = 0;
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        //     0  1   2    3  4 5  6
        // ex: mx dec -934 if w == 4068
        let (reg, op, val) = (parts[0], parts[1], parts[2]);
        let (test_reg, comp_op, test_val) = (parts[4], parts[5], parts[6]);

        let test_reg_val = *registers.entry(test_reg).or_insert(0);
        let test_val: i32 = test_val.parse().unwrap();
        
        let is_true = match comp_op {
            "==" => test_reg_val == test_val,
            "!=" => test_reg_val != test_val,
            ">" => test_reg_val > test_val,
            "<" => test_reg_val < test_val,
            ">=" => test_reg_val >= test_val,
            "<=" => test_reg_val <= test_val,
            _ => unreachable!("Invalid comparison!")
        };

        // didn't match, don't do assignment
        if !is_true { continue; }

        let val: i32 = val.parse().unwrap();
        let reg = registers.entry(reg).or_insert(0);
        *reg += match op {
            "inc" => val,
            "dec" => -val,
            _ => unreachable!("Only inc/dec supported")
        };

        if *reg > max {
            max = *reg;
        }
    }

    // find max register value
    let end_max = registers.keys().map(|key| registers[key]).max().unwrap();

    (max, end_max)
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let (part_one_solution, part_two_solution) = part_one_and_two(puzzle);

    println!("Solution to part one is {}", part_one_solution);
    println!("Solution to part two is {}", part_two_solution);
}
