use std::collections::HashSet;
use nalgebra::{point, vector, Vector2};

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

const DIRECTIONS: [Vector2<i32>; 8] = [
    vector![ 1,  0],
    vector![ 1,  1],
    vector![ 0,  1],
    vector![-1,  1],
    vector![-1,  0],
    vector![-1, -1],
    vector![ 0, -1],
    vector![ 1, -1],
];

fn main() {
    // Read map
    let mut rolls = HashSet::new();

    for (y, line) in read_lines!("day-04/input.txt").enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                rolls.insert(point![x as i32, y as i32]);
            }
        }
    }

    // Part 1
    let mut removed = 0;

    for step in 0.. {
        let removable: Vec<_> = rolls.iter()
            .filter(|&roll| {
                let cnt = DIRECTIONS.iter()
                    .filter(|&v| rolls.contains(&(roll + v)))
                    .count();

                cnt < 4
            })
            .copied()
            .collect();

        println!("step {step:>2}: {}", removable.len());

        removed += removable.len();

        for roll in &removable {
            rolls.remove(roll);
        }

        if removable.len() == 0 {
            break;
        }
    }

    println!("part 02: {}", removed);
}
