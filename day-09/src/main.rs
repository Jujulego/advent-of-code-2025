use nalgebra::{max, point, Point2};

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
    let mut tiles = read_lines!("day-09/input.txt")
        .map(|line| {
            let mut point = point![0, 0];
            let mut coords = line.split(',').map(|coord| coord.parse::<i64>().unwrap());

            point.x = coords.next().unwrap();
            point.y = coords.next().unwrap();

            point
        })
        .collect::<Vec<_>>();

    tiles.sort_by(|a, b|
        a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y))
    );

    // Search the largest area
    let mut largest_area = 0;

    for (idx, a) in tiles.iter().enumerate() {
        for b in tiles[idx + 1..].iter().rev() {
            largest_area = max(largest_area, area(a, b))
        }
    }

    println!("part 01: {}", largest_area);
}

fn area(a: &Point2<i64>, b: &Point2<i64>) -> u64 {
    ((a.x - b.x).unsigned_abs() + 1) * ((a.y - b.y).unsigned_abs() + 1)
}