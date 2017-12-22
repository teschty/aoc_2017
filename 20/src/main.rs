use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read_file() -> String {
    let mut contents = String::new();
    let mut f = File::open("puzzle.txt").expect("puzzle.txt couldn't be found :(");
    f.read_to_string(&mut contents).expect("Couldn't read puzzle.txt :(");

    contents
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    pub pos: Vec3,
    vel: Vec3,
    acc: Vec3
}

impl Particle {
    fn simulate(&mut self) {
        self.vel.add(&self.acc);
        self.pos.add(&self.vel);
    }
}

fn part_one(input: &str) -> usize {
    let mut particles = vec![];
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
    let last = particles[0].acc.distance();

    for particle in particles {
        if particle.acc.distance() != last { break }
        lowest_acc.push(particle);
    }

    // then sort by velocity
    lowest_acc.sort_by(|a, b| a.vel.distance().cmp(&b.vel.distance()));
    lowest_acc[0].num
}

fn part_two(input: &str) -> usize {
    let mut particles = vec![];
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

    for _ in 0..100 {
        let mut sites = HashMap::new();
        let mut to_destroy: Vec<usize> = vec![];

        for (i, particle) in particles.iter_mut().enumerate() {
            if let Some(&particle_idx) = sites.get(&particle.pos) {
                to_destroy.push(particle_idx);
                to_destroy.push(i);
            } else {
                sites.insert(particle.pos, i);
            }

            particle.simulate();
        }

        to_destroy.sort();
        // remove duplicate values
        to_destroy.dedup();

        for &i in to_destroy.iter().rev() {
            particles.remove(i);
        }
    }

    particles.len()
}

fn main() {
    let puzzle = read_file();
    let puzzle = puzzle.trim();
    
    println!("Solution to part one is {}", part_one(puzzle));
    println!("Solution to part two is {}", part_two(puzzle));
}
