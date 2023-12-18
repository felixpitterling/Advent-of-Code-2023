const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}


pub fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, holding) = numbers.split_once("|").unwrap();
        let winning = winning.split_whitespace();
        let holding: Vec<_> = holding.split_whitespace().collect();
        let num_winners = winning.filter(|s| holding.contains(s)).count() as u32;
        let score = match num_winners {
            0 => 0,
            n => 2u32.pow(n - 1),
        };
        sum += score;
    }
    sum
}