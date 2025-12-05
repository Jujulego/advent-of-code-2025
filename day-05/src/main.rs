use std::cmp::max;
use std::ops::RangeInclusive;

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
    let mut lines = read_lines!("day-05/input.txt");
    let mut fresh_ranges = Vec::new();

    // Parsing ranges
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split('-')
            .map(|part| part.parse::<u64>().unwrap());

        let start = parts.next().unwrap();
        let end = parts.next().unwrap();

        fresh_ranges.push(start..=end);

        continue;
    }

    // Reducing ranges
    let mut reduced_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    fresh_ranges.sort_by_key(|range| *range.start());

    for range in fresh_ranges {
        if let Some(prev) = reduced_ranges.last_mut() {
            if prev.contains(range.start()) {
                *prev = *prev.start()..=max(*prev.end(), *range.end());
                continue;
            }
        }

        reduced_ranges.push(range.clone());
    }

    // Part 1
    let mut part1 = 0;

    for id in lines.map(|line| line.parse::<u64>().unwrap()) {
        let is_fresh = reduced_ranges.iter()
            .any(|range| range.contains(&id));

        if is_fresh {
            part1 += 1;
        }
    }

    // Part 2
    let part2 = reduced_ranges.iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<u64>();

    println!("part 01: {}", part1);
    println!("part 02: {}", part2);
}
