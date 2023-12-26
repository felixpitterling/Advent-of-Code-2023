const INPUT: &str = include_str!("./../input.txt");
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    // | is a vertical pipe connecting north and south.
    NorthSouth,
    // - is a horizontal pipe connecting east and west.
    EastWest,
    // L is a 90-degree bend connecting north and east.
    NorthEast,
    // J is a 90-degree bend connecting north and west.
    NorthWest,
    // 7 is a 90-degree bend connecting south and west.
    SouthWest,
    // F is a 90-degree bend connecting south and east.
    SouthEast,
    // . is ground; there is no pipe in this tile.
    Ground,
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    Start,
}
use Tile::*;
impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

impl Coord {
    fn new(row_idx: usize, col_idx: usize) -> Self {
        Self { row_idx, col_idx }
    }

    fn valid_neighbours(&self, map: &[Vec<Tile>]) -> Vec<Coord> {
        let mut neighbours = vec![];
        let max_height = map.len() - 1;
        let max_width = map[0].len() - 1;

        match map[self.row_idx][self.col_idx] {
            Ground => (),
            Start => {
                // north
                if self.row_idx > 0 {
                    let tile = map[self.row_idx - 1][self.col_idx];
                    if matches!(tile, NorthSouth | SouthWest | SouthEast) {
                        neighbours.push(Coord::new(self.row_idx - 1, self.col_idx));
                    }
                }
                // south
                if self.row_idx < max_height {
                    let tile = map[self.row_idx + 1][self.col_idx];
                    if matches!(tile, NorthSouth | NorthWest | NorthEast) {
                        neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                    }
                }
                // west
                if self.col_idx > 0 {
                    let tile = map[self.row_idx][self.col_idx - 1];
                    if matches!(tile, EastWest | SouthEast | NorthEast) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                    }
                }
                // east
                if self.col_idx < max_width {
                    let tile = map[self.row_idx][self.col_idx + 1];
                    if matches!(tile, EastWest | NorthWest | SouthWest) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                    }
                }
            }
            NorthSouth => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // south
                if self.row_idx < max_height && map[self.row_idx + 1][self.col_idx] != Ground {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
            }
            EastWest => {
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            NorthEast => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            NorthWest => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            SouthWest => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            SouthEast => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
        }

        neighbours
    }
}

fn main() {
    let answer = part_1(INPUT);
    println!("Answer: {}", answer);
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coord) {
    let mut start = Coord::new(0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    let tile = Tile::from(c);
                    if tile == Start {
                        start = Coord::new(row_idx, col_idx)
                    }
                    tile
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn build_loop(start: Coord, map: &[Vec<Tile>]) -> HashSet<Coord> {
    let mut loop_coords = HashSet::new();
    loop_coords.insert(start);
    let mut to_visit = start.valid_neighbours(map);

    while let Some(curr_pos) = to_visit.pop() {
        for neighbour in curr_pos.valid_neighbours(map) {
            if !loop_coords.contains(&neighbour) {
                to_visit.push(neighbour);
                loop_coords.insert(neighbour);
            }
        }
    }

    loop_coords
}

pub fn part_1(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    loop_coords.len() / 2
}