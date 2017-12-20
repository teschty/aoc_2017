use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

#[derive(Clone, Copy)]
struct Vec3 {
    x: i32, y: i32, z: i32
}

impl Vec3 {
    fn add(&mut self, other: &Vec3) {
        self.x += other.x; 
        self.y += other.y;
        self.z += other.z;
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

struct Particle {
    num: usize,

    pos: Vec3,
    vel: Vec3,
    acc: Vec3
}

impl Particle {
    fn simulate(&mut self) {
        self.vel.add(&self.acc);
        self.pos.add(&self.vel);
    }

    fn distance(&self) -> i32 {
        self.pos.distance()
    }
}

fn part_one(input: &str) -> usize {
    let mut particles = vec![];
    let mut closest_long_run = 0;
    let mut idx = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(">, ").collect();

        let mut vecs = vec![];
        for part in parts {
            let nums: Vec<i32> = part
                .split(|c| c == ',' || c == '<' || c == '>')
                .map(|f| f.parse())
                .filter_map(Result::ok)
                .collect();

            vecs.push(Vec3 { x: nums[0], y: nums[1], z: nums[2] });
        }

        particles.push(Particle {
            num: idx,

            pos: vecs[0],
            vel: vecs[1],
            acc: vecs[2]
        });

        idx += 1;
    }

    particles.sort_by(|a, b| a.acc.distance().cmp(&b.acc.distance()));
    let mut lowest_acc = vec![];
    let mut last = particles[0].acc.distance();

    for particle in particles {
        if particle.acc.distance() != last { break }
        lowest_acc.push(particle);
    }

    // then sort by velocity
    lowest_acc.sort_by(|a, b| a.vel.distance().cmp(&b.vel.distance()));
    lowest_acc[0].num
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();
    
    println!("Solution to part one is {}", part_one(puzzle));
}
