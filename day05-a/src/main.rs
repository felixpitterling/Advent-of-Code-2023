const INPUT: &str = include_str!("./../input.txt");

struct Rule {
    destination: i64,
    source: i64,
    range: i64,
}

fn part_1(input: &str) -> i64 {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds = seeds.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let mut maps = Vec::new();
    for block in maps_str.split("\n\n") {
        let (_, rules) = block.split_once("\n").unwrap();
        let mut map = Vec::new();
        for line in rules.lines() {
            let mut nums = line.splitn(3, " ");
            let destination: i64 = nums.next().unwrap().parse().unwrap();
            let source: i64 = nums.next().unwrap().parse().unwrap();
            let range: i64 = nums.next().unwrap().parse().unwrap();
            map.push(Rule {
                destination,
                source,
                range,
            });
        }
        maps.push(map);
    } 

    let mut min = i64::MAX;

    for seed in seeds{
        let mut curr = seed;

        'maps: for map in &maps {
            for rule in map {
                let rule_applies = curr >= rule.source && curr <= rule.source + rule.range;
                if rule_applies {
                    let offset = curr - rule.source;
                    curr = rule.destination + offset;
                    continue 'maps;
                }
            }
        }
        min = min.min(curr);
    }
    min
}
 
fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}
