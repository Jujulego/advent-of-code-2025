use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter::zip;

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
        let leds: i32 = leds[1..leds.len() - 1]
            .char_indices()
            .map(|(idx, l)| if l == '#' { 1 << idx } else { 0 })
            .sum();

        // Parse required joltage
        let required_joltage = parts.next_back().unwrap();
        let required_joltage = required_joltage[1..required_joltage.len() - 1]
            .split(',')
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Parse buttons
        let buttons = parts
            .map(|btn| btn[1..btn.len() - 1]
                .split(',')
                .map(|p| p.parse::<u8>().unwrap())
                .map(|i| 1 << i)
                .sum::<i32>()
            )
            .collect::<Vec<i32>>();

        machines.push(Machine { leds, buttons, required_joltage });
    }

    // Start machines
    let part1 = machines.iter()
        .map(|machine| {
            // DFS
            let mut marks: HashSet<i32> = HashSet::from([0]);

            let mut queue = VecDeque::new();
            queue.push_back(LedStep { num: 0, state: 0 });

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
                    queue.push_back(LedStep { num, state })
                }
            }

            unreachable!();
        })
        .sum::<i32>();

    println!("part 01: {}", part1);

    // Give required joltage
    let mut part2 = Vec::new();

    for machine in &machines {
        println!("machine: {:?}", machine);
        let mut marks = HashMap::new();
        marks.insert(vec![0; machine.required_joltage.len()], 0);

        let mut queue = BinaryHeap::new();
        queue.push(JoltageStep {
            num: 0,
            state: vec![0; machine.required_joltage.len()],
            press_map: HashMap::new(),
        });

        while let Some(step) = queue.pop() {
            // println!("step: {:?}", step);
            // Shortcut was found
            if marks.get(&step.state).is_some_and(|n| n < &step.num) {
                continue;
            }

            // Ended ?
            if step.state == machine.required_joltage {
                println!("min: {:?}", step);
                part2.push(step.num);
                break;
            }

            // Evaluate missing
            let missing = zip(&machine.required_joltage, &step.state)
                .map(|(r, s)| r - s)
                .collect::<Vec<_>>();
            // println!("missing: {:?}", missing);

            for button in &machine.buttons {
                let cnts = missing.iter()
                    .enumerate()
                    .filter_map(|(idx, v)| {
                        let b = (button >> idx) & 0b1;

                        if b == 0 || (step.press_map.get(&v).unwrap_or(&0) + v) < 0 {
                            None
                        } else {
                            Some(v)
                        }
                    })
                    .filter(|&v| v > &0);

                for cnt in cnts {
                    // println!("button: {:?} x {:?}", button, cnt);

                    let state = increase_joltage(&step.state, *button, *cnt);
                    let num = step.num + cnt;

                    if zip(&machine.required_joltage, &state).find(|&(r, v)| v > r).is_some() {
                        continue;
                    }

                    if marks.get(&state).is_some_and(|n| n <= &num) {
                        continue;
                    }

                    marks.insert(state.clone(), num);

                    let mut press_map = step.press_map.clone();
                    press_map.entry(*button)
                        .and_modify(|e| *e += cnt)
                        .or_insert(*cnt);

                    queue.push(JoltageStep { num, state, press_map });
                }
            }
        }
    }

    println!("part 02: {}", part2.iter().sum::<i32>());
    assert_eq!(part2.len(), machines.len());
}

#[derive(Debug)]
struct Machine {
    leds: i32,
    buttons: Vec<i32>,
    required_joltage: Vec<i32>,
}

struct LedStep {
    num: i32,
    state: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct JoltageStep {
    num: i32,
    state: Vec<i32>,
    press_map: HashMap<i32, i32>,
}

impl Ord for JoltageStep {
    fn cmp(&self, other: &Self) -> Ordering {
        other.num.cmp(&self.num)
    }
}

impl PartialOrd for JoltageStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn increase_joltage(state: &[i32], button: i32, cnt: i32) -> Vec<i32> {
    state.iter()
        .enumerate()
        .map(|(idx, v)| {
            let b = (button >> idx) & 0b1;
            v + (b * cnt)
        })
        .collect()
}
