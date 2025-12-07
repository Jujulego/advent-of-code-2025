use std::collections::{HashSet, VecDeque};
use nalgebra::{point, vector};

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn main() {
    let mut start = point![0, 0];
    let mut splitters = HashSet::new();
    let mut end_y = 0;

    for (y, line) in read_lines!("day-07/input.txt").enumerate() {
        for (x, char) in line.char_indices() {
            match char {
                'S' => {
                    start = point![x, y].cast::<i32>();
                },
                '^' => {
                    splitters.insert(point![x, y].cast::<i32>());
                },
                '.' => continue,
                _ => unreachable!(),
            }
        }

        end_y = y as i32;
    }

    // Search impacted splitters
    let mut stack = VecDeque::from([start]);
    let mut visited = HashSet::new();
    let mut marks = HashSet::new();

    while let Some(point) = stack.pop_back() {
        let next = point + vector![0, 1];

        if point.y >= end_y { // Reached the end
            continue;
        }

        // Mark position
        if marks.contains(&next) {
            continue;
        }

        marks.insert(next.clone());

        // Interact
        if splitters.contains(&next) {
            visited.insert(next.clone());

            stack.push_back(next + vector![ 1, 0]);
            stack.push_back(next + vector![-1, 0]);
        } else {
            stack.push_back(next);
        }
    }

    println!("part 01: {}", visited.len());
}
