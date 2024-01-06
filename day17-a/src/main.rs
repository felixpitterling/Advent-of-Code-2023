use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Crucible {
    cost: u32,
    pos: Coord,
    dir: Direction,
    moves_in_dir: u8,
}

impl Crucible {
    fn successors(&self, grid: &Vec<Vec<u8>>) -> Vec<Self> {
        let rows = grid.len();
        let cols = grid[0].len();
        
        let mut successors = Vec::new();
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            ] {
                if self.dir == dir && self.moves_in_dir == 3 {
                    continue;
                }
                if self.dir.opposite() == dir {
                    continue;
                }
                if let Some(pos) = self.pos.forward(&dir, rows, cols) {
                    let cost = self.cost + grid[pos.row][pos.col] as u32;
                    
                    let moves_in_dir = if self.dir == dir {
                        self.moves_in_dir + 1
                } else {
                    1
                };
                
                successors.push(Crucible {
                    pos,
                    cost,
                    dir,
                    moves_in_dir,
                })
            }
        }
        successors
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn forward(&self, dir: &Direction, rows: usize, cols: usize) -> Option<Self> {
        let coord = match dir {
            Direction::Up if self.row > 0 => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down if self.row < (rows - 1) => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left if self.col > 0 => Coord {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right if self.col < (cols - 1) => Coord {
                row: self.row,
                col: self.col + 1,
            },
            _ => return None,
        };
        Some(coord)
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn part_1(input: &str) -> u32 {
    let grid = parse(input);
    let goal = Coord {
        row: grid.len() - 1,
        col: grid[0].len() - 1,
    };

    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    let right = Crucible {
        cost: grid[0][1] as u32,
        dir: Direction::Right,
        pos: Coord { row: 0, col: 1 },
        moves_in_dir: 1,
    };
    pq.push(right);

    let down = Crucible {
        cost: grid[1][0] as u32,
        dir: Direction::Down,
        pos: Coord { row: 1, col: 0 },
        moves_in_dir: 1,
    };
    pq.push(down);

    while let Some(crucible) = pq.pop() {
        if crucible.pos == goal {
            return crucible.cost;
        }
        for crucible in crucible.successors(&grid) {
            if seen.insert((crucible.pos, crucible.dir, crucible.moves_in_dir)) {
                pq.push(crucible);
            }
        }
    }
    panic!("No path found")
}