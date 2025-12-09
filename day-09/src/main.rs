use nalgebra::{max, min, point, Point2};
use std::cmp::Ordering;
use std::collections::HashMap;
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
    // Load tiles
    let red_tiles = read_lines!("day-09/input.txt")
        .map(|line| {
            let mut point = point![0, 0];
            let mut coords = line.split(',').map(|coord| coord.parse::<i64>().unwrap());

            point.x = coords.next().unwrap();
            point.y = coords.next().unwrap();

            point
        })
        .collect::<Vec<_>>();

    let min_x = red_tiles.iter().map(|tile| tile.x).min().unwrap();

    // Search the largest area
    let mut largest_area = 0;

    for (idx, a) in red_tiles.iter().enumerate() {
        for b in red_tiles[idx + 1..].iter().rev() {
            largest_area = max(largest_area, area(a, b))
        }
    }

    println!("part 01: {}", largest_area);

    // Build borders
    let mut borders: HashMap<i64, RangeInclusive<i64>> = HashMap::new();

    for (idx, a) in red_tiles.iter().enumerate() {
        add_point(&mut borders, a);

        let b = if idx + 1 == red_tiles.len() { &red_tiles[0] } else { &red_tiles[idx + 1] };

        if a.x == b.x {
            let y_range = match a.y.cmp(&b.y) {
                Ordering::Less => (a.y + 1)..b.y,
                Ordering::Greater => (b.y + 1)..a.y,
                Ordering::Equal => unreachable!(),
            };

            y_range.map(|y| point![a.x, y])
                .for_each(|pt| add_point(&mut borders, &pt));
        } else {
            let x_range = match a.x.cmp(&b.x) {
                Ordering::Less => (a.x + 1)..b.x,
                Ordering::Greater => (b.x + 1)..a.x,
                Ordering::Equal => unreachable!(),
            };

            x_range.map(|x| point![x, a.y])
                .for_each(|pt| add_point(&mut borders, &pt));
        }
    }

    // Search the largest area
    let mut largest_area = 0;

    for (idx, a) in red_tiles.iter().enumerate() {
        for b in red_tiles[idx + 1..].iter().rev() {
            let y_check = (min(a.y, b.y)..=max(a.y, b.y))
                .map(|y| [point![a.x, y], point![b.x, y]])
                .flatten()
                .all(|pt| is_point_inside(&borders, &pt));

            if y_check {
                largest_area = max(largest_area, area(a, b));
            }
        }
    }

    println!("part 02: {}", largest_area);
}

fn area(a: &Point2<i64>, b: &Point2<i64>) -> u64 {
    ((a.x - b.x).unsigned_abs() + 1) * ((a.y - b.y).unsigned_abs() + 1)
}

type BordersMap = HashMap<i64, RangeInclusive<i64>>;

fn add_point(borders: &mut BordersMap, pt: &Point2<i64>) {
    borders.entry(pt.y)
        .and_modify(|range| {
            *range = *min(range.start(), &pt.x)..=*max(range.end(), &pt.x)
        })
        .or_insert(pt.x..=pt.x);
}

fn is_point_inside(borders: &BordersMap, point: &Point2<i64>) -> bool {
    borders.get(&point.y)
        .is_some_and(|range| range.contains(&point.x))
}