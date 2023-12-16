const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_2(INPUT);
    println!("Answer: {}", answer);
}


fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let (_, draws) = line.split_once(": ").unwrap();
        for draw in draws.split("; ") {
            for pair in draw.split(", ") {
                let (num, color) = pair.split_once(" ").unwrap();
                let num: u32 = num.parse().unwrap();
                match color {
                    "red" => min_red = min_red.max(num),
                    "green" => min_green = min_green.max(num),
                    "blue" => min_blue = min_blue.max(num),
                    _ => panic!("at the disco"),
                }
            }
        }
        sum += min_red * min_green * min_blue;
    }
    sum
}


