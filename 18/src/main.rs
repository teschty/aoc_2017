use std::fs::File;
use std::io::Read;
use std::collections::LinkedList;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

struct Program<'a> {
    registers: [i64; 26],
    instructions: &'a Vec<Vec<&'a str>>,
    ip: usize,
    pub send_queue: LinkedList<i64>,
    pub times_sent: i32
}

impl<'a> Program<'a> {
    fn get_value(&self, operand: &str) -> i64 {
        let c = operand.chars().nth(0).unwrap();

        if c.is_alphabetic() {
            self.registers[c as usize - 97]
        } else {
            operand.parse().unwrap()
        }
    }

    // returns true if waiting
    fn run(&mut self, receive_queue: &mut LinkedList<i64>) -> bool {
        let mut jumped = false;
        let ins = &self.instructions[self.ip];

        let x = reg_idx(ins[1]);
        let y = if ins.len() > 2 {
            self.get_value(ins[2])
        } else {
            0
        };

        match ins[0] {
            "snd" => {
                self.send_queue.push_back(self.registers[x]);
                self.times_sent += 1;
            }

            "set" => self.registers[x] = y,
            "add" => self.registers[x] += y,
            "mul" => self.registers[x] *= y,
            "mod" => self.registers[x] %= y,
            "rcv" => {
                match receive_queue.pop_front() {
                    Some(val) => self.registers[x] = val,
                    None => return true
                }
            }
            "jgz" => {
                // w o w
                let val = if ins[1].chars().nth(0).unwrap().is_alphabetic() {
                    self.registers[x]
                } else {
                    x as i64
                };

                if val > 0 && y != 0 { 
                    jumped = true;
                    self.ip = (self.ip as i64 + y) as usize;
                }
            }

            _ => panic!("Unknown instruction {}", ins[0])
        }

        if !jumped {
            self.ip += 1;
        }

        if self.ip >= self.instructions.len() {
            panic!("Went past instructions");
        }

        false
    }

    fn new(id: i64, instructions: &'a Vec<Vec<&'a str>>) -> Self {
        let mut program = Program {
            registers: [0; 26],
            instructions,
            ip: 0,
            send_queue: LinkedList::new(),
            times_sent: 0
        };

        program.registers[reg_idx("p")] = id;
        program
    }
}

// returns index of register from character value
fn reg_idx(register: &str) -> usize {
    let reg_char = register.chars().nth(0).unwrap();
    if reg_char.is_alphabetic() {
        reg_char as usize - 97
    } else {
        register.parse().unwrap()
    }
}

fn part_two(input: &str) -> i32 {
    let instructions: Vec<Vec<&str>> = input.lines()
        .map(|s| s.split(' ').collect())
        .collect();

    let mut prog0 = Program::new(0, &instructions);
    let mut prog1 = Program::new(1, &instructions);

    loop {
        let blocked1 = prog0.run(&mut prog1.send_queue);
        let blocked2 = prog1.run(&mut prog0.send_queue);

        if blocked1 && blocked2 {
            break;
        }
    }

    prog1.times_sent
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    println!("Solution to part two is {}", part_two(puzzle));
}
