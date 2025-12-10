use std::collections::{HashSet, VecDeque};

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
    let mut machines = Vec::new();

    for line in read_lines!("day-10/input.txt") {
        let mut parts = line.split_whitespace();

        // Parse leds
        let leds = parts.next().unwrap();
        let leds: u32 = leds[1..leds.len() - 1]
            .char_indices()
            .map(|(idx, l)| if l == '#' { 1 << idx } else { 0 })
            .sum();

        // Parse required joltage
        let required_joltage = parts.next_back().unwrap();
        let required_joltage = required_joltage[1..required_joltage.len() - 1]
            .split(',')
            .map(|l| l.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        // Parse buttons
        let buttons = parts
            .map(|btn| btn[1..btn.len() - 1]
                .split(',')
                .map(|p| p.parse::<u8>().unwrap())
                .map(|i| 1 << i)
                .sum::<u32>()
            )
            .collect::<Vec<u32>>();

        machines.push(Machine { leds, buttons, required_joltage });
    }

    let part1 = machines.iter()
        .map(|machine| {
            // DFS
            let mut marks: HashSet<u32> = HashSet::from([0]);

            let mut queue = VecDeque::new();
            queue.push_back(Step { num: 0, state: 0 });

            while let Some(step) = queue.pop_front() {
                let num = step.num + 1;

                for btn in &machine.buttons {
                    let state = step.state ^ btn;

                    if state == machine.leds {
                        return num;
                    }

                    if marks.contains(&state) {
                        continue;
                    }

                    marks.insert(state);
                    queue.push_back(Step { num, state })
                }
            }

            unreachable!();
        })
        .sum::<u32>();

    println!("part 01: {}", part1);
}

struct Machine {
    leds: u32,
    buttons: Vec<u32>,
    required_joltage: Vec<u32>,
}

struct Step {
    num: u32,
    state: u32,
}