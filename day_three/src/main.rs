use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn part_one(input: i32) -> i32 {
    // won't work for 1, so hardcode result
    if input == 1 { return 0; }

    let (mut x, mut y) = (1i32, 0i32);
    let mut layer = 0;
    let mut current_num = 2;

    let mut dir = Direction::Right;
    let mut distance = 0;

    // fill with initial instructions
    while current_num != input {
        if distance == 0 {
            match dir {
                Direction::Up => {
                    dir = Direction::Left;
                    distance = 2 * layer;
                },
                Direction::Down => {
                    dir = Direction::Right;
                    distance = 2 * layer + 1;
                },
                Direction::Left => {
                    dir = Direction::Down;
                    distance = 2 * layer;
                },
                Direction::Right => {
                    dir = Direction::Up;
                    // new layer
                    layer += 1;
                    distance = 2 * layer - 1;
                }
            }
        }

        match dir {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1
        }

        distance -= 1;
        current_num += 1;
    }

    x.abs() + y.abs()
}

use std::collections::HashMap;

fn part_two(input: i32) -> i32 {
    let (mut x, mut y) = (1i32, 0i32);
    let mut layer = 0;
    let mut current_num = 1;

    let mut dir = Direction::Right;
    let mut distance = 0;
    
    let mut values = HashMap::new();
    values.insert((0, 0), 1);

    // fill with initial instructions
    while current_num <= input {
        if distance == 0 {
            match dir {
                Direction::Up => {
                    dir = Direction::Left;
                    distance = 2 * layer;
                },
                Direction::Down => {
                    dir = Direction::Right;
                    distance = 2 * layer + 1;
                },
                Direction::Left => {
                    dir = Direction::Down;
                    distance = 2 * layer;
                },
                Direction::Right => {
                    dir = Direction::Up;
                    // new layer
                    layer += 1;
                    distance = 2 * layer - 1;
                }
            }
        }

        let mut value = 0;

        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 { continue; }

                value += match values.get(&(x + i, y + j)) {
                    Some(val) => *val,
                    None => 0
                }
            }
        }

        current_num = value;
        values.insert((x, y), current_num);

        match dir {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1
        }

        distance -= 1;
    }

    current_num
}

fn main() {
    println!("Enter the puzzle input: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read from stdin!");
    let input: i32 = input.trim().parse().expect("Couldn't convert to i32!");

    let part_one_solution = part_one(input);
    let part_two_solution = part_two(input);
    println!("Part one solution is {}", part_one_solution);
    println!("Part two solution is {}", part_two_solution);
}
