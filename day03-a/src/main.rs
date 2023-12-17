const INPUT: &str = include_str!("./../input.txt");

pub fn part_1(input: &str) -> u32 {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut current_num = 0;
    let mut has_adjacent_symbol = false;

    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[row_idx].len() {
            let value = grid[row_idx][col_idx];

            // If not a number, continue
            if !value.is_ascii_digit() {
                continue;
            }

            // check if any adjacent tile is a symbol
            for row_offset in -1..=1 {
                for col_offset in -1..=1 {
                    //Skip itself
                    if row_offset == 0 && col_offset == 0 {
                        continue;
                    }

                    let adjacent_row_idx = row_idx as i32 + row_offset;
                    let adjacent_col_idx = col_idx as i32 + col_offset;

                    // Out of bounds
                    if adjacent_row_idx < 0
                        || adjacent_row_idx >= grid.len() as i32
                        || adjacent_col_idx < 0
                        || adjacent_col_idx >= grid[adjacent_row_idx as usize].len() as i32
                    {
                        continue;
                    }

                    let adjacent_value = grid[adjacent_row_idx as usize][adjacent_col_idx as usize];
                    if !adjacent_value.is_ascii_digit() && adjacent_value != '.' {
                        has_adjacent_symbol = true;
                    }
                }
            }

            // Adjust the number currently being built
            current_num *= 10;
            current_num += value.to_digit(10).unwrap();

            // If we reached EOL or the next horizontal cord is not a digit, the current num is complete
            if col_idx + 1 >= grid[row_idx].len() || !grid[row_idx][col_idx + 1].is_ascii_digit() {
                if has_adjacent_symbol {
                    sum += current_num;
                }

                current_num = 0;
                has_adjacent_symbol = false;
            }
        }
    }
    sum
}

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}
