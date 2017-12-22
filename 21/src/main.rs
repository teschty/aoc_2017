use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

type Grid = Vec<Vec<bool>>;

fn rotate(pattern: Grid) -> Vec<Grid> {
    let rotated = vec![];
    if(pattern.len() == 2) {
        let (q0, q1, q2, q3) = (pattern[0][0], pattern[1][0], pattern[1][1], pattern[0][1]);
        
    }

    rotated
}

fn part_one(input: &String) {

}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();


}
