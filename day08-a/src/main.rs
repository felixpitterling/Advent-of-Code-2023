use std::collections::HashMap;

const INPUT: &str = include_str!("./../input.txt");

enum Instruction {
    Left,
    Right
}

struct Destinations<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part_1(input: &str) -> u32 {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().map(|c| match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("at the disco"),
    });
    let map: HashMap<&str, Destinations> = map
        .lines()
        .map(|line| {
            let (source, destinations) = line.split_once(" = ").unwrap();
            let destinations = destinations
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap();
            let destinations = destinations.split_once(", ").unwrap();
            (
                source,
                Destinations {
                    left: destinations.0,
                    right: destinations.1,
                },
            )
        })  
        .collect();

    let mut instructions = instructions.cycle();
    let mut steps = 0;
    let mut curr = "AAA";

    while curr != "ZZZ" {
        let ins = instructions.next().unwrap();
        let Destinations { left, right } = map.get(curr).unwrap();
        curr = match ins {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        steps += 1;
    }
    steps
}


fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}