use itertools::Itertools;
use std::cmp::Ordering;

const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_2(INPUT);
    println!("Answer: {}", answer);
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand, bid)
        })
        .sorted_by(|(a, _), (b, _)| compare_hands2(a, b))
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid.parse::<usize>().unwrap())
        .sum()
}

fn compare_hands2(a: &str, b: &str) -> Ordering {
    hand_score2(a).cmp(&hand_score2(b)).then_with(|| {
        let (a, b) = a
            .chars()
            .zip(b.chars())
            .find(|(a, b)| a != b)
            .expect("hands are completely identical");

        card_value2(a).cmp(&card_value2(b))
    })
}

fn hand_score2(hand: &str) -> [u8; 2] {
    let mut faces = [0; 13];
    let mut jokers = 0;
    for c in hand.chars() {
        if c == 'J' {
            jokers += 1;
        } else {
            faces[card_value2(c)] += 1;
        }
    }
    faces.sort_unstable();
    let mut score: [u8; 2] = faces[11..].try_into().unwrap();
    score.reverse();
    // add the amount of jokers to the counts of the card that occurs the most already to increase the hand score
    score[0] += jokers;
    score
}

fn card_value2(c: char) -> usize {
    "J23456789TQKA".chars().position(|card| card == c).unwrap()
}