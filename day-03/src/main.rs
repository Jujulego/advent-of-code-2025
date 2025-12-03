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

    for line in read_lines!("day-03/input.txt") {
        let batteries = line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        let mut max = 0;

        for (idx, b1) in batteries.iter().enumerate() {
            for b2 in &batteries[(idx + 1)..] {
                let joltage = b1 * 10 + b2;

                max = max.max(joltage);

                if *b2 == 9 {
                    break;
                }
            }
        }

        part1 += max;
    }

    println!("part 01: {}", part1);
}
