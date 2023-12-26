const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_2(INPUT);
    println!("Answer: {}", answer);
}

pub fn part_2(input: &str) -> i32 {
    let mut nums: Vec<i32> = Vec::new();
    let mut differences: Vec<i32> = Vec::new();
    let mut edge: Vec<i32> = Vec::new();

    input
        .lines()
        .map(|line| {
            nums.clear();
            for num in line.split_ascii_whitespace() {
                nums.push(num.parse().unwrap());
            }

            edge.clear();

            loop {
                differences.clear();
                for i in nums.windows(2) {
                    differences.push(i[0] - i[1]);
                }
                edge.push(nums[0]);
                if differences.iter().all(|&x| x == 0) {
                    let sum: i32 = edge.iter().sum();
                    break sum;
                }
                std::mem::swap(&mut nums, &mut differences);
            }
        })
        .sum()
}