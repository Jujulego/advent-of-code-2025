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
    let input = read_lines!("day-02/input.txt").next().unwrap();
    let mut part1 = 0;
    let mut part2 = 0;

    for range in input.split(',') {
        let mut parts = range.split('-');
        let start = parts.next().unwrap().parse::<u64>().unwrap();
        let end = parts.next().unwrap().parse::<u64>().unwrap();

        for value in start..=end {
            let dim = value.ilog10() + 1;

            // Part 1
            if dim % 2 == 0 {
                let half = dim / 2;
                let pow = TEN.pow(half);

                if value % pow == value / pow {
                    // println!("{}", value);
                    part1 += value;
                }
            }

            // Part 2
            'test: for cnt in (2..=dim).filter(|x| dim % x == 0) {
                let pow = TEN.pow(dim / cnt);

                let mut d = value;
                let pattern = d % pow;

                while d - pattern > 0 {
                    d /= pow;

                    if d % pow != pattern {
                        continue 'test;
                    }
                }

                println!("{}", value);
                part2 += value;
                break;
            }
        }
    }

    println!("part 01: {}", part1);
    println!("part 02: {}", part2);
}
