use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

enum Direction {
    Up(),
    Down(),
    Left(),
    Right(),
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_map<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>> {
        let mut map = Vec::<Vec<char>>::new();
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                continue;
            }
            map.push(line.chars().collect());
        }
        Ok(map)
    }

    fn find_guard(map: &Vec<Vec<char>>) -> (usize, usize) {
        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '^' {
                    return (x, y);
                }
            }
        }
        panic!("Guard not found");
    }

    fn traverse_map(map: &mut Vec<Vec<char>>) -> bool {
        let (mut guard_x, mut guard_y) = find_guard(&map);
        let mut step_count = 0;
        let mut dir = Direction::Up();
        loop {
            map[guard_y][guard_x] = 'X';
            if guard_y == 0
                || guard_y == map.len() - 1
                || guard_x == 0
                || guard_x == map[0].len() - 1
            {
                break;
            }

            if step_count > map.len() * map[0].len() * 3 {
                return true;
            }

            match dir {
                Direction::Up() => {
                    if map[guard_y - 1][guard_x] == '#' {
                        dir = Direction::Right();
                    } else {
                        guard_y -= 1;
                        step_count += 1;
                    }
                }
                Direction::Down() => {
                    if map[guard_y + 1][guard_x] == '#' {
                        dir = Direction::Left();
                    } else {
                        guard_y += 1;
                        step_count += 1;
                    }
                }
                Direction::Left() => {
                    if map[guard_y][guard_x - 1] == '#' {
                        dir = Direction::Up();
                    } else {
                        guard_x -= 1;
                        step_count += 1;
                    }
                }
                Direction::Right() => {
                    if map[guard_y][guard_x + 1] == '#' {
                        dir = Direction::Down();
                    } else {
                        guard_x += 1;
                        step_count += 1;
                    }
                }
            }
        }
        return false;
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map = parse_map(reader)?;
        traverse_map(&mut map);

        let mut x_count = 0;
        for row in map.iter() {
            x_count += row.iter().filter(|&&c| c == 'X').count();
        }
        Ok(x_count)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_map(reader)?;

        let mut possible_pos = 0;

        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell != '#' && *cell != '^' {
                    let mut map_copy = map.clone();
                    map_copy[y][x] = '#';
                    if traverse_map(&mut map_copy) {
                        possible_pos += 1;
                    }
                }
            }
        }
        Ok(possible_pos)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
