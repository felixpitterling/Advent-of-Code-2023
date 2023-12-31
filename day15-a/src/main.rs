const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, byte| acc.wrapping_add(byte).wrapping_mul(17))
}

pub fn part_1(input: &str) -> u32 {
    input.trim().split(',').map(|s| hash(s) as u32).sum()
}