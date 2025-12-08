use nalgebra::{point, Point3};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};

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
    // Load boxes
    let boxes = read_lines!("day-08/input.txt")
        .map(|line| {
            let mut pt = point![0, 0, 0];
            let mut c = line.split(",").map(|x| x.parse::<i64>().unwrap());

            pt.x = c.next().unwrap();
            pt.y = c.next().unwrap();
            pt.z = c.next().unwrap();

            pt
        })
        .collect::<Vec<_>>();

    // Prepare pairs
    println!("Building pairs ...");
    let mut pairs = BTreeSet::new();

    for (idx, a) in boxes.iter().enumerate() {
        for b in &boxes[idx + 1..] {
            pairs.insert(Pair(a.clone(), b.clone()));
        }
    }

    // Build circuits
    println!("Building circuits ...");
    let mut circuits: Vec<HashSet<Point3<i64>>> = vec![];
    let mut leftover: HashSet<Point3<i64>> = HashSet::from_iter(boxes);

    for (idx, Pair(point_a, point_b)) in pairs.iter().enumerate() {
        print!("({:>5}, {:>5}, {:>5}) -> ({:>5}, {:>5}, {:>5}) ({:>8})",
                 point_a.x, point_a.y, point_a.z,
                 point_b.x, point_b.y, point_b.z,
                 Pair(*point_a, *point_b).distance_squared()
        );

        let circuit_a = circuits.iter()
            .enumerate()
            .find(|(_, c)| c.contains(point_a));

        if let Some((mut idx_a, circuit_a)) = circuit_a {
            if circuit_a.contains(point_b) {
                println!();
            } else {
                let circuit_b = circuits.iter()
                    .enumerate()
                    .find(|(_, c)| c.contains(point_b));

                if let Some((idx_b, _)) = circuit_b {
                    // Merge circuits
                    let circuit_b = circuits.remove(idx_b);
                    idx_a = if idx_b < idx_a { idx_a - 1 } else { idx_a };

                    circuits[idx_a].extend(circuit_b);

                    println!(" => merge");
                } else {
                    // Add point b to circuit a
                    circuits[idx_a].insert(*point_b);
                    leftover.remove(point_b);

                    println!(" => add b");
                }
            }
        } else {
            let circuit_b = circuits.iter_mut()
                .find(|c| c.contains(point_b));

            if let Some(circuit_b) = circuit_b {
                // Add point a to circuit b
                circuit_b.insert(*point_a);
                leftover.remove(point_a);

                println!(" => add a");
            } else {
                // Add new circuit
                circuits.push(HashSet::from([*point_a, *point_b]));
                leftover.remove(point_a);
                leftover.remove(point_b);

                println!(" => new");
            }
        }

        if idx == 999 {
            let part1: usize = circuits.iter()
                .rev()
                .take(3)
                .map(|circuit| circuit.len())
                .product();

            println!("part 01: {}", part1);
        }

        if leftover.is_empty() {
            let part2 = point_a.x * point_b.x;

            println!("part 02: {}", part2);
            break;
        }
    }

    println!("Done !");
}

#[derive(Debug, Eq, PartialEq)]
struct Pair(Point3<i64>, Point3<i64>);

impl Pair {
    fn distance_squared(&self) -> i64 {
        let d = self.0 - self.1;
        d.x * d.x + d.y * d.y + d.z * d.z
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Pair) -> Ordering {
        self.distance_squared().cmp(&other.distance_squared())
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Pair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
