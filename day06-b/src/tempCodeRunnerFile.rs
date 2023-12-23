const INPUT: &str = include_str!("./../input.txt");



fn main() {
    let answer = part_2(INPUT);
    println!("Answer: {}", answer);
}

pub fn part_2(input: &str) -> u32 {
    let (time, dist) = input.split_once("\n").unwrap();
    let race_time = time
        .strip_prefix("Time: ")
        .unwrap()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0u64, |curr, digit| curr * 10 + digit as u64);
    let race_dist = dist
        .strip_prefix("Distance: ")
        .unwrap()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0u64, |curr, digit| curr * 10 + digit as u64);

    let a = 1.0;
    let b = 0.0 - race_time as f64;
    let c = race_dist as f64;

    let x1 = ((0.0 - b) - (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);
    let x2 = ((0.0 - b) + (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);

    let lower_bound = x1.ceil() as u32;
    let upper_bound = x2.floor() as u32 + 1;

    upper_bound - lower_bound
}

