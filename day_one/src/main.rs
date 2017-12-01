use std::fs::File;
use std::io::Read;

// reads in puzzle.txt
fn read_file() -> String {
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

fn main() {
    let puzzle = read_file();
    // trim to remove newline
    let puzzle = puzzle.trim();
    
    // since these are just all ASCII numbers (0-9)
    // we can just convert this to byte array
    let bytes = puzzle.as_bytes();

    let mut sum = 0i32;
    for i in 0..bytes.len() {
        // eh, don't like this long conditional
        if (i + 1 == bytes.len() && bytes[i] == bytes[0]) || bytes[i] == bytes[i + 1] {
            let value = bytes[i] as i32;
            // convert to actual value by subtracting ASCII value of '0'
            let value = value - '0' as i32;

            sum += value;
        }
    }

    println!("Captcha solution is {}", sum);
}
