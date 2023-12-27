use itertools::Itertools;

struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}

pub fn part_1(input: &str) -> usize {
    parse(input).map(|record| record.valid_arrangements()).sum()
}

fn parse(input: &str) -> impl Iterator<Item = Record> + '_ {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("at the disco"),
            })
            .collect();
        let counts = counts.split(',').map(|s| s.parse().unwrap()).collect();

        Record { springs, counts }
    })
}

impl Record {
    fn is_valid(&self) -> bool {
        self.springs
            .iter()
            .group_by(|item| *item)
            .into_iter()
            .filter_map(|(key, group)| {
                if *key == Spring::Damaged {
                    Some(group.count())
                } else {
                    None
                }
            })
            .eq(self.counts.iter().copied())
    }


    fn valid_arrangements(&self) -> usize {
        if let Some(index) = self
            .springs
            .iter()
            .position(|spring| *spring == Spring::Unknown)
        {
            let mut as_damaged_spring = self.springs.clone();
            as_damaged_spring[index] = Spring::Damaged;
            let as_damaged = Record {
                springs: as_damaged_spring,
                counts: self.counts.to_vec(),
            };

            let mut as_operational_spring = self.springs.clone();
            as_operational_spring[index] = Spring::Operational;
            let as_operational = Record {
                springs: as_operational_spring,
                counts: self.counts.to_vec(),
            };

            as_damaged.valid_arrangements() + as_operational.valid_arrangements()
        } else {
            if self.is_valid() {
                1
            } else {
                0
            }
        }
    }
}
