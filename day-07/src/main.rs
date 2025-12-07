use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use nalgebra::{point, vector, Point2};

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
    let mut timelines = HashMap::new();
    let mut paths = HashMap::new();

    while let Some(point) = stack.pop_back() {
        // Mark timeline
        match timelines.entry(point) {
            Entry::Occupied(entry) => {
                let inc = *entry.get();
                increment_timelines(point, &paths, &mut timelines, inc);
                continue;
            },
            Entry::Vacant(entry) => {
                entry.insert(0);
            }
        }

        // Reached the end
        if point.y >= end_y {
            timelines.insert(point, 1);
            increment_timelines(point, &paths, &mut timelines, 1);
            continue;
        }

        // Interact
        let next = point + vector![0, 1];

        if splitters.contains(&next) {
            visited.insert(point);

            let right = point + vector![1, 1];
            paths.insert(right, point);
            stack.push_back(right);

            let left = point + vector![-1, 1];
            paths.insert(left, point);
            stack.push_back(left);
        } else {
            paths.insert(next, point);
            stack.push_back(next);
        }
    }

    println!("part 01: {}", visited.len());
    println!("part 02: {}", timelines.get(&start).unwrap());
}

fn increment_timelines(
    mut point: Point2<i32>,
    paths: &HashMap<Point2<i32>, Point2<i32>>,
    timelines: &mut HashMap<Point2<i32>, usize>,
    inc: usize,
) {
    while let Some(&previous) = paths.get(&point) {
        point = previous;
        *timelines.get_mut(&point).unwrap() += inc;
    }
}