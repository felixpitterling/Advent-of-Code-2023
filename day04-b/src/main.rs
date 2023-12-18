const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_2(INPUT);
    println!("Answer: {}", answer);
}


pub fn part_2(input: &str) -> u32 {
    let mut counts = vec![1; input.lines().count()];

    for (idx, line) in input.lines().enumerate() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, holding) = numbers.split_once("|").unwrap();
        let winning = winning.split_whitespace();
        let holding: Vec<_> = holding.split_whitespace().collect();
        let num_winners = winning.filter(|s| holding.contains(s)).count();

        // update the card counts for every card we win
        let num_cards = counts[idx];
        for i in (idx + 1)..=(idx + num_winners) {
            counts[i] += num_cards;
        }
    }
    counts.iter().sum()
}