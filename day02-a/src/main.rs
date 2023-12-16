const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}


fn part_1(input: &str) -> usize {
    let mut sum = 0;
    'game: for (idx, line) in input.lines().enumerate() {
        let id = idx + 1;
        let (_, draws) = line.split_once(": ").unwrap();
        for draw in draws.split("; ") {
            // check if a draw is possible
            for pair in draw.split(", ") {
                let (num, color) = pair.split_once(" ").unwrap();
                let num: u32 = num.parse().unwrap();
                let possible = match color {
                    "red" => num <= 12,
                    "green" => num <= 13,
                    "blue" => num <= 14,
                    _ => panic!("at the disco"),
                };
                if !possible {
                    continue 'game;
                }
            }
        }
        sum += id;
    }
    sum
}


