
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
    let mut part2 = 0;

    for line in read_lines!("day-01/input.txt") {
        let cnt: i32 = (&line[1..]).parse().unwrap();

        if &line[0..1] == "R" {
            for _ in 0..cnt {
                state += 1;

                if state == 100 {
                    state = 0;
                    part2 += 1;
                }
            }
        } else {
            for _ in 0..cnt {
                state -= 1;

                if state == 0 {
                    part2 += 1;
                }

                if state == -1 {
                    state = 99;
                }
            }
        }

        if state == 0 {
            part1 += 1;
        }

        println!("{line:<4} => {state:<3}  {part1:<4}  {part2:<4}");
    }

    println!("part 01: {part1}");
    println!("part 02: {part2}");
}
