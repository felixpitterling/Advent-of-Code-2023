#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Round,
    Square,
    Empty,
}

const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_2(INPUT);
    println!("Answer: {}", answer);
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Square,
                    'O' => Tile::Round,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

fn slide_north(grid: &mut Vec<Vec<Tile>>) {
    for col in 0..grid[0].len() {
        let mut empty_or_round_row = 0;
        for row in 0..grid.len() {
            let curr = grid[row][col];
            match curr {
                Tile::Square => empty_or_round_row = row + 1,
                Tile::Round => {
                    // swap the current tile with the empty_or_round one
                    let replace_with = std::mem::replace(&mut grid[empty_or_round_row][col], curr);
                    let _ = std::mem::replace(&mut grid[row][col], replace_with);
                    empty_or_round_row += 1;
                }
                Tile::Empty => (),
            }
        }
    }
}

fn weight(grid: &Vec<Vec<Tile>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let round_rocks = row.iter().filter(|tile| **tile == Tile::Round).count();
            round_rocks * (i + 1)
        })
        .sum()
}

// rotate 90 degrees clockwise: (x, y) -> (y, -x)
fn clockwise(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let size = grid.len();
    let mut rotated = vec![vec![Tile::Empty; size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated[col][size - 1 - row] = grid[row][col];
        }
    }
    rotated
}

fn cycle(mut grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..4 {
        slide_north(&mut grid);
        let rotated = clockwise(&grid);
        grid = rotated;
    }
    grid
}


pub fn part_2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut seen = vec![grid.clone()];

    loop {
        grid = cycle(grid);
        if let Some(idx) = seen.iter().position(|x| x == &grid) {
            let cycle_len = seen.len() - idx;
            let final_idx = idx + (1_000_000_000 - idx) % cycle_len;
            return weight(&seen[final_idx]);
        }
        seen.push(grid.clone());
    }
}