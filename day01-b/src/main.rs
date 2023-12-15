const INPUT: &str = include_str!("./../input.txt");

#[derive(Debug, PartialEq)]
struct NumberPosition {
    number: u32,
    position: usize,
}

// "1a2b3c" -> [NumberPosition{ number: 1, position: 0 }, NumberPosition{}, ...]
fn find_digits(input: &str) -> Vec<NumberPosition> {
    input   
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(i, c)| NumberPosition {
            number: c.to_digit(10).unwrap(),
            position: i,
        })
        .collect()
}

fn find_spelled_numbers(input: &str) -> Vec<NumberPosition> {
    let spelled_numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut positions: Vec<NumberPosition> = Vec::new();
    for spelled_number in spelled_numbers.clone() {
        let mut start = 0;
        while let Some(position) = input[start..].find(spelled_number) {
            positions.push(NumberPosition {
                number: spelled_numbers.iter().position(|&n| n == spelled_number).unwrap() as u32 + 1,
                position: start + position,
            });
            start += position + spelled_number.len();
        }
    }
    positions.sort_by_key(|np| np.position);
    positions
}

pub fn solve(input: &str) {
    let ans = input.lines().map(|line| get_numbers_from_line(line)).sum::<u32>();
    println!("Answer: {}", ans);
}

fn get_numbers_from_line(line: &str) -> u32 {
    let digits = find_digits(line);
    let spelled_numbers = find_spelled_numbers(line);
    let mut numbers: Vec<NumberPosition> = digits;
    numbers.extend(spelled_numbers);
    numbers.sort_by_key(|np| np.position);

    let first = numbers.first().expect("there should be at least one number");
    let last = numbers.last().expect("there should be at least one number");
    let answer = first.number * 10 + last.number;

    //println!("{} -> {:?} + {:?} = {}", line, first, last, answer);

    answer
}

fn main() {
    solve(INPUT); 
}
