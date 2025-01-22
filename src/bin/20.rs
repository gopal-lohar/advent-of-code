use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    fn parse_map<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>> {
        let mut map = Vec::<Vec<char>>::new();
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                continue;
            }
            let mut map_line = Vec::<char>::new();
            for c in line.chars() {
                map_line.push(c);
            }
            map.push(map_line);
        }
        Ok(map)
    }

    fn traverse_map(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let mut path = Vec::<(usize, usize)>::new();
        let mut position = (0, 0);
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'S' {
                    position = (x, y);
                    break;
                }
            }
        }

        path.push(position);
        let mut previous_position = position;
        let mut index = 0;
        while map[position.1][position.0] != 'E' {
            let mut next_position = position;
            if index == 0 {
                if position.0 > 0 && map[position.1][position.0 - 1] != '#' {
                    next_position = (position.0 - 1, position.1);
                } else if position.0 < map[0].len() - 1 && map[position.1][position.0 + 1] != '#' {
                    next_position = (position.0 + 1, position.1);
                } else if position.1 > 0 && map[position.1 - 1][position.0] != '#' {
                    next_position = (position.0, position.1 - 1);
                } else if position.1 < map.len() - 1 && map[position.1 + 1][position.0] != '#' {
                    next_position = (position.0, position.1 + 1);
                }
            } else {
                if position.0 > 0
                    && map[position.1][position.0 - 1] != '#'
                    && (position.0 - 1, position.1) != previous_position
                {
                    next_position = (position.0 - 1, position.1);
                } else if position.0 < map[0].len() - 1
                    && map[position.1][position.0 + 1] != '#'
                    && (position.0 + 1, position.1) != previous_position
                {
                    next_position = (position.0 + 1, position.1);
                } else if position.1 > 0
                    && map[position.1 - 1][position.0] != '#'
                    && (position.0, position.1 - 1) != previous_position
                {
                    next_position = (position.0, position.1 - 1);
                } else if position.1 < map.len() - 1
                    && map[position.1 + 1][position.0] != '#'
                    && (position.0, position.1 + 1) != previous_position
                {
                    next_position = (position.0, position.1 + 1);
                }
            }
            path.push(position);
            position = next_position;
            previous_position = position;

            index += 1;
        }
        path.push(position);

        path
    }

    fn check_cheat(
        map: &Vec<Vec<char>>,
        path: &Vec<(usize, usize)>,
        position_index: usize,
        time_to_save: usize,
    ) -> bool {
        println!("position_index: {}", position_index);
        let position = path[position_index];
        let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (index, next_move) in moves.iter().enumerate() {
            let next_position = (
                position.0 as isize + next_move.0,
                position.1 as isize + next_move.1,
            );
            if next_position.1 > 0
                && next_position.1 < map.len() as isize - 1
                && next_position.0 > 0
                && next_position.0 < map[0].len() as isize - 1
            {
                for (i, move2) in moves.iter().enumerate() {
                    if i == index {
                        continue;
                    }
                    let next_position2 = (
                        next_position.0 as isize + move2.0,
                        next_position.1 + move2.1,
                    );
                    if next_position2.1 > 0
                        && next_position2.1 < map.len() as isize - 1
                        && next_position2.0 > 0
                        && next_position2.0 < map[0].len() as isize - 1
                    {
                        let next_position2 = (next_position2.0 as usize, next_position2.1 as usize);
                        if map[next_position2.1][next_position2.0] == '.' {
                            for next_in_path in position_index + time_to_save..path.len() {
                                if path[next_in_path] == next_position2 {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_map(reader)?;
        let path = traverse_map(&map);
        println!("traversed the map");
        for index in 0..path.len() {
            check_cheat(&map, &path, index, 12);
        }
        Ok(1)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(8, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
