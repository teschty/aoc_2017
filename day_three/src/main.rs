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
                    layer += 1;
                    distance = if layer == 1 { 1 } else { 2 * layer - 1 };
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

    println!("Coords = ({}, {})", x, y);

    x.abs() + y.abs()
}

fn main() {
    println!("Enter the puzzle input: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read from stdin!");
    let input: i32 = input.trim().parse().expect("Couldn't convert to i32!");

    let part_one_solution = part_one(input);
    println!("Part one solution is {}", part_one_solution);
}
