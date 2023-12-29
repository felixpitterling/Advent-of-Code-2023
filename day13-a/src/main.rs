use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}

fn parse(input: &str) -> Vec<VecDeque<Vec<Tile>>> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Tile::Ash,
                            '#' => Tile::Rock,
                            _ => panic!("at the disco"),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn reflects_at(grid: &VecDeque<Vec<Tile>>) -> Option<usize> {
    (1..grid.len()).find(|&offset| {
        let half1 = grid.iter().take(offset).rev();
        let half2 = grid.iter().skip(offset);
        let mut combined = half1.zip(half2);
        combined.all(|(row1,row2)| row1 == row2)
    })
}

pub fn part_1(input: &str) -> usize {
    let grid = parse(input);
    grid.iter()
        .map(|grid| {
            //check horizontal
            if let Some(i) = reflects_at(grid) {
                return i * 100;
            }

            //check vertical
            let cols = (0..grid[0].len())
                .map(|i| grid.iter().map(|row| row[i]).collect())
                .collect();
            if let Some(i) = reflects_at(&cols) {
                return i;
            }

            0
        })
        .sum()
}