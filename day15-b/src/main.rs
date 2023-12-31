const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_2(INPUT);
    println!("Answer: {}", answer);
}

enum Instruction<'a> {
    Remove(&'a str),
    Add(Lens<'a>),
}

impl<'a> Instruction<'a> {
    fn new(s: &'a str) -> Self {
        if let Some(label) = s.strip_suffix('-') {
            Self::Remove(label)
        } else {
            let (label, focal) = s.split_once('=').unwrap();
            let focal = focal.parse().unwrap();
            let lens = Lens { label, focal };
            Self::Add(lens)
        }
    }
}

struct Lens<'a> {
    label: &'a str,
    focal: u8,
}

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, byte| acc.wrapping_add(byte).wrapping_mul(17))
}

pub fn part_2(input: &str) -> usize {
    const BOX: Vec<Lens> = Vec::new();
    let mut boxes = [BOX; 256];

    for instr in input.trim_end().split(',').map(Instruction::new) {
        match instr {
            Instruction::Remove(label) => {
                let hash = hash(label);
                boxes[hash as usize].retain(|item| item.label != label);
            }
            Instruction::Add(lens) => {
                let hash = hash(lens.label);
                let lenses = &mut boxes[hash as usize];
                if let Some(old) = lenses.iter_mut().find(|item| lens.label == item.label) {
                    // update focal length of lens with this label
                    old.focal = lens.focal;
                } else {
                    // add lens to end of box
                    lenses.push(lens);
                }
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_idx, lenses)| {
            let box_focusing_power: usize = lenses
                .into_iter()
                .enumerate()
                .map(|(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * lens.focal as usize)
                .sum();
            box_focusing_power
        })
        .sum()
}