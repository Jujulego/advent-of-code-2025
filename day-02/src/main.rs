macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

const TEN: u64 = 10;

fn main() {
    let mut part1 = 0;
    let input = read_lines!("day-02/input.txt").next().unwrap();

    for range in input.split(',') {
        let mut parts = range.split('-');
        let start = parts.next().unwrap().parse::<u64>().unwrap();
        let end = parts.next().unwrap().parse::<u64>().unwrap();

        for value in start..=end {
            let dim = value.ilog10() + 1;

            if dim % 2 != 0 {
                continue;
            }

            let half = dim / 2;
            let pow = TEN.pow(half);

            if value % pow == value / pow {
                println!("{}", value);
                part1 += value;
            }
        }
    }

    println!("part 01: {}", part1);
}
