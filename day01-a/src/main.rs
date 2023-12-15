const INPUT: &str = include_str!("./../input.txt");

fn main() {
    solve(INPUT);
}

pub fn solve(input: &str) {
    // iterate over each line and get the two digits in the string
    let ans: u32 = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_digit(10));

            let first = chars
                .next()
                .expect("the line should have at least one digit");
            let num = match chars.last() {
                Some(last) => format!("{}{}", first, last),
                None => format!("{}{}", first, first),
            };
            num.parse::<u32>().unwrap()
        })
        .sum();
    println!("Answer: {}", ans)
}