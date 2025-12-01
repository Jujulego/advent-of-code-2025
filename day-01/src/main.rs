
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
    let mut state = 50;
    let mut part1 = 0;

    for line in read_lines!("day-01/input.txt") {
        let cnt = &line[1..].parse().unwrap();

        if &line[0..1] == "R" {
            state = (state + cnt) % 100;
        } else {
            state = (state - cnt + 100) % 100;
        }

        // println!("{line} => {state}");

        if state == 0 {
            part1 += 1;
        }
    }

    println!("part 01: {part1}");
}
