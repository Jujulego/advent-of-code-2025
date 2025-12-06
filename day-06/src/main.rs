use std::ops::Range;

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
    let lines: Vec<_> = read_lines!("day-06/input.txt").collect();

    let numbers = &lines[0..lines.len()-1];
    let mut signs = &lines.last().unwrap()[..];

    // Extract problems
    let mut problems = Vec::new();

    while let Some(idx) = rfind_sign(signs) {
        problems.push(Problem {
            sign: signs[idx..].trim(),
            range: idx..signs.len(),
        });

        signs = &signs[..idx.saturating_sub(1)];
    }

    // Part 1
    let mut part1 = 0;

    for problem in &problems {
        let terms = numbers.iter()
            .map(|terms| terms[problem.range.clone()]
                .trim()
                .parse::<u64>()
                .unwrap()
            );

        part1 += match problem.sign {
            "+" => terms.sum::<u64>(),
            "*" => terms.product::<u64>(),
            _ => unreachable!("Unknown sign {}", problem.sign),
        }
    }

    println!("part 01: {}", part1);
}

#[derive(Debug)]
struct Problem<'a> {
    sign: &'a str,
    range: Range<usize>,
}

fn rfind_sign(line: &str) -> Option<usize> {
    [line.rfind('+'), line.rfind('*')].iter()
        .filter_map(|&idx| idx)
        .max()
}