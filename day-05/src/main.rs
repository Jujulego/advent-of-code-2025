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

    // Paring ids
    let mut part1 = 0;

    for id in lines.map(|line| line.parse::<u64>().unwrap()) {
        let is_fresh = fresh_ranges.iter()
            .any(|range| range.contains(&id));

        if is_fresh {
            part1 += 1;
        }
    }

    println!("part 01: {}", part1);
}
