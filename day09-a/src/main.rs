use itertools::Itertools;
const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}


pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

        let mut edge: Vec<i32> = Vec::new();
        loop {
            let differences: Vec<i32> = nums
                .iter()
                .tuple_windows()
                .map(|(left, right)| right - left)
                .collect();

            edge.push(*nums.last().unwrap());

            if differences.iter().all(|&x| x == 0) {
                let sum: i32 = edge.iter().sum();
                break sum;
            } else {
                nums = differences;
            }
                
        }
        })
        .sum()
}