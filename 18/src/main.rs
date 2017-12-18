use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn reg_idx(register: &str) -> usize {
    let reg_char = register.chars().nth(0).unwrap();
    reg_char as usize - 97
}

fn get_value(registers: &[i64; 26], operand: &str) -> i64 {
    let c = operand.chars().nth(0).unwrap();

    if c.is_alphabetic() {
        registers[c as usize - 97]
    } else {
        operand.parse().unwrap()
    }
}

fn part_one(input: &str) -> i64 {
    let instructions: Vec<Vec<&str>> = input.lines()
        .map(|s| s.split(' ').collect())
        .collect();

    let mut ip = 0;
    let mut last_freq = 0;
    let recovered_freq;
    // eh just make 26
    let mut registers = [0; 26];

    loop {
        let mut jumped = false;
        let ins = &instructions[ip];

        let x = reg_idx(ins[1]);
        let y = if ins.len() > 2 {
            get_value(&registers, ins[2])
        } else {
            0
        };

        match ins[0] {
            "snd" => last_freq = registers[x],
            "set" => registers[x] = y,
            "add" => registers[x] += y,
            "mul" => registers[x] *= y,
            "mod" => registers[x] %= y,
            "rcv" => {
                if registers[x] != 0 {
                    recovered_freq = last_freq;
                    break;
                }
            }
            "jgz" => {
                if registers[x] != 0 && y != 0 { 
                    jumped = true;
                    ip = (ip as i64 + y) as usize;
                }
            }

            _ => panic!("Unknown instruction {}", ins[0])
        }

        if !jumped {
            ip += 1;
        }
    }

    recovered_freq
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part one is {}", part_one(puzzle));
}
