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
    // Parse input
    let lines: Vec<_> = read_lines!("day-06/input.txt")
        .map(|line| line.split_whitespace()
            .map(|part| part.to_string())
            .collect::<Vec<_>>()
        )
        .collect();

    let numbers: Vec<_> = lines[0..lines.len() - 1].iter()
        .map(|terms| terms.iter()
            .map(|term| term.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
        )
        .collect();

    let signs = &lines[lines.len() - 1];

    assert!(numbers.iter().all(|terms| terms.len() == signs.len()));

    // Part 1
    let mut part1 = 0;

    for i in 0..signs.len() {
        let terms = numbers.iter().map(|terms| terms[i]);
        let sign = &signs[i][0..1];

        part1 += match sign {
            "+" => terms.sum::<u64>(),
            "*" => terms.product::<u64>(),
            _ => unreachable!("Unknown sign {sign}")
        }
    }

    println!("part 01: {}", part1);
}
