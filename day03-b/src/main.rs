const INPUT: &str = include_str!("./../input.txt");
use std::collections::{HashMap, HashSet};

fn main() {
    let answer = part_2(INPUT);
    println!("Answer: {}", answer);
}

pub fn part_2(input: &str) -> u32 {
    let engine = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // key: star coordinate, val: list of adjacent numbers
    let mut stars: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    let mut current_num = 0;
    let mut adjacent_star_positions: HashSet<(i32, i32)> = HashSet::new();

    for row_idx in 0..engine.len() {
        for col_idx in 0..engine[row_idx].len() {
            let value = engine[row_idx][col_idx];

            // Not a number, not interested
            if !value.is_ascii_digit() {
                continue;
            }

            // check if any adjacent tile is a star
            for row_offset in -1..=1 {
                for col_offset in -1..=1 {
                    // Skip self
                    if row_offset == 0 && col_offset == 0 {
                        continue;
                    }

                    let adjacent_row_idx = row_idx as i32 + row_offset;
                    let adjacent_col_idx = col_idx as i32 + col_offset;

                    // Out of bounds
                    if adjacent_row_idx < 0
                        || adjacent_row_idx >= engine.len() as i32
                        || adjacent_col_idx < 0
                        || adjacent_col_idx >= engine[adjacent_row_idx as usize].len() as i32
                    {
                        continue;
                    }

                    if engine[adjacent_row_idx as usize][adjacent_col_idx as usize] == '*' {
                        adjacent_star_positions.insert((adjacent_row_idx, adjacent_col_idx));
                    }
                }
            }

            // Adjust the number currently being built (concatenate a digit using math)
            current_num *= 10;
            current_num += value.to_digit(10).unwrap();

            // If we reached the end of the line, or the next horizontal coordinate is not a digit, the current number is complete
            if col_idx + 1 >= engine[row_idx].len()
                || !engine[row_idx][col_idx + 1].is_ascii_digit()
            {
                // add all stars to the variable keeping track of stars (potential gears)
                for point in &adjacent_star_positions {
                    stars.entry(*point).or_default().push(current_num);
                }

                // reset the temporary values before starting on a new number
                current_num = 0;
                adjacent_star_positions.clear();
            }
        }
    }

    stars
        .values()
        // only stars with exactly 2 adjacent numbers are gears
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .sum()
}
