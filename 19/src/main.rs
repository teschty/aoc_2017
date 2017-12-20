use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up, Down, Left, Right
}

fn is_pipe(c: char) -> bool {
    c == '|' || c == '-'
}

fn find_new_dir(path: &Vec<Vec<char>>, x: usize, y: usize, prev_dir: Direction) -> Direction {
    let top = y > 0 && is_pipe(path[y - 1][x]) && prev_dir != Direction::Down;
    let left = x > 0 && is_pipe(path[y][x - 1]) && prev_dir != Direction::Right;
    let bottom = y < path.len() && is_pipe(path[y + 1][x]) && prev_dir != Direction::Up;
    let right = x < path[0].len() && is_pipe(path[y][x + 1]) && prev_dir != Direction::Left;

    if top {
        Direction::Up
    } else if bottom {
        Direction::Down
    } else if left {
        Direction::Left
    } else if right {
        Direction::Right
    } else {
        panic!("Could not find new direction!")
    }
}

fn part_one_and_two(input: &str) -> (String, i32) {
    let mut result = String::new();

    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start = input[0].iter().position(|&c| c == '|').unwrap();
    let mut dir = Direction::Down;

    let (mut x, mut y, mut steps) = (start, 0, 0);
    while x >= 0 && x < input[0].len() && y >= 0 && y < input.len() {
        // change direction
        match input[y][x] {
            // new direction
            '+' => dir = find_new_dir(&input, x, y, dir),

            '|' | '-' => {},

            c if c.is_alphabetic() => result.push(c),

            _ => break
        }

        match dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1
        }
        
        steps += 1;
    }

    (result, steps)
}

fn main() {
    let puzzle = read_file();
    let (part_one_solution, part_two_solution) = part_one_and_two(&puzzle);
    println!("Solution to part one is {}", part_one_solution);
    println!("Solution to part two is {}", part_two_solution);
}
