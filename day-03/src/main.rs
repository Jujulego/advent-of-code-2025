use std::collections::{BinaryHeap, HashMap};

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
    let mut part1 = 0;
    let mut part2 = 0;

    for line in read_lines!("day-03/input.txt") {
        let batteries = line.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        part1 += find_battery_setup(&batteries, 2);
        part2 += find_battery_setup(&batteries, 12);
    }

    println!("part 01: {}", part1);
    println!("part 02: {}", part2);
}

#[derive(Debug, Eq, PartialEq)]
struct BatterySetup {
    priority: u64,
    joltage: u64,
    idx: usize,
}

impl Ord for BatterySetup {
    fn cmp(&self, other: &BatterySetup) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for BatterySetup {
    fn partial_cmp(&self, other: &BatterySetup) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_battery_setup(batteries: &[u64], limit: u32) -> u64 {
    let mut heap = BinaryHeap::new();
    let mut marks = HashMap::new();
    let mut max = 0;

    heap.push(BatterySetup { priority: 0, joltage: 0, idx: 0 });

    while let Some(setup) = heap.pop() {
        if setup.priority < max {
            continue;
        }

        for (idx, b) in batteries[setup.idx..].iter().enumerate() {
            let joltage = setup.joltage * 10 + *b;

            if marks.get(&joltage).is_some_and(|i| i <= &idx) {
                continue;
            }

            marks.insert(joltage, idx);

            let dim = joltage.ilog10();

            if dim + 1 == limit {
                max = max.max(joltage);
            } else {
                heap.push(BatterySetup {
                    priority: joltage * (10u64.pow(limit - dim - 1)),
                    joltage,
                    idx: setup.idx + idx + 1,
                });
            }
        }
    }

    max
}