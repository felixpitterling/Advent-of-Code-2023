const INPUT: &str = include_str!("./../input.txt");

struct Race {
    time: u32,
    dist: u32,
}

pub fn part_1(input: &str) -> usize {
    let (time, dist) = input.split_once("\n").unwrap();
    let time = time
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap());
    let dist = dist
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap());
    let races = time.zip(dist).map(|(time, dist)| Race { time, dist });

    races
        .map(|race| {
            (0..=race.time)
                .map(|elapsed| {
                        let speed = elapsed;
                        speed * (race.time - elapsed)
                })
                .filter(|&dist| dist > race.dist)
                .count()
        })
        .product::<usize>()
}


fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}
