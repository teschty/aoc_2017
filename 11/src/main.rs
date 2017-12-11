use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn distance(x: i32, y: i32) -> i32 {
    if x == 0 {
        y.abs()
    } else if y == 0 {
        x.abs()
    } else {
        let dx = x.abs();
        let dy = y.abs();

        if y >= 0 {
            dx + dy - ((dx as f32 / 2.0).ceil() as i32)
        } else {
            dx + dy -  ((dx as f32 / 2.0).floor() as i32)
        }
    }
}

fn part_one_and_two(input: &str) -> (i32, i32) {
    let instructions: Vec<&str> = input.split(',').collect();

    let (mut x, mut y) = (0i32, 0i32);
    let mut max_distance = 0;

    for instruction in instructions {
        let even = x % 2 == 0;
        match instruction {
            "n" => y += 1,
            "s" => y -= 1,
            "nw" => { 
                y += if even { 1 } else { 0 };
                x -= 1;
            }
            "ne" => { 
                y += if even { 1 } else { 0 };
                x += 1;
            }
            "sw" => { 
                y -= if !even { 1 } else { 0 };
                x -= 1;
            }
            "se" => { 
                y -= if !even { 1 } else { 0 };
                x += 1;
            }
            _ => panic!("Unknown direction {}", instruction)
        }

        max_distance = max_distance.max(distance(x, y));
    }

    (distance(x, y), max_distance)
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();

    let (part_one_solution, part_two_solution) = part_one_and_two(puzzle);

    println!("Solution to part one is {}", part_one_solution);
    println!("Solution to part two is {}", part_two_solution);
}
