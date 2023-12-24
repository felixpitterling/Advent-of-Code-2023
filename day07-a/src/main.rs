use itertools::Itertools;
use std::cmp::Ordering;

const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand, bid)
        })
        .sorted_by(|(a, _), (b, _)| compare_hands(a, b))
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid.parse::<usize>().unwrap())
        .sum()
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    hand_score(a).cmp(&hand_score(b)).then_with(|| {
        let (a, b) = a
            .chars()
            .zip(b.chars())
            .find(|(a, b)| a != b)
            .expect("hands are completely identical");

        card_value(a).cmp(&card_value(b))
    })
}

fn hand_score(hand: &str) -> [u8; 2] {
    let mut faces = [0_u8; 13];
    for c in hand.chars() {
        faces[card_value(c)] += 1;
    }
    faces.sort_unstable();
    let mut score: [u8; 2] = faces[11..].try_into().unwrap();
    score.reverse();
    score
}

fn card_value(c: char) -> usize {
    "23456789TJQKA".chars().position(|card| card == c).unwrap()
}