use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[derive(PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
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

    fn get_antennas(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Position>> {
        let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '.' {
                    continue;
                }
                antennas.entry(*cell).or_default().push(Position { x, y });
            }
        }
        antennas
    }

    fn is_valid_position(x: i32, y: i32, map_width: usize, map_height: usize) -> bool {
        x >= 0 && x < map_width as i32 && y >= 0 && y < map_height as i32
    }

    fn get_antenna_antinodes(
        pos1: &Position,
        pos2: &Position,
        map_width: usize,
        map_height: usize,
        all_positions: bool,
    ) -> Vec<Position> {
        let delta_x = pos2.x as i32 - pos1.x as i32;
        let delta_y = pos2.y as i32 - pos1.y as i32;

        let mut atenna_antinodes = Vec::<Position>::new();

        if (pos1.x as i32) + delta_x == (pos2.x as i32) {
            let mut x = (pos1.x as i32) - delta_x;
            let mut y = (pos1.y as i32) - delta_y;

            if is_valid_position(x, y, map_width, map_height) {
                atenna_antinodes.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
            }

            while all_positions && is_valid_position(x, y, map_width, map_height) {
                atenna_antinodes.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
                x -= delta_x;
                y -= delta_y;
            }
        } else {
            let mut x = (pos1.x as i32) + delta_x;
            let mut y = (pos1.y as i32) + delta_y;

            if is_valid_position(x, y, map_width, map_height) {
                atenna_antinodes.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
            }

            while all_positions && is_valid_position(x, y, map_width, map_height) {
                atenna_antinodes.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
                x += delta_x;
                y += delta_y;
            }
        }

        if (pos2.x as i32) + delta_x == (pos1.x as i32) {
            let mut x = (pos2.x as i32) - delta_x;
            let mut y = (pos2.y as i32) - delta_y;

            if is_valid_position(x, y, map_width, map_height) {
                atenna_antinodes.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
            }

            while all_positions && is_valid_position(x, y, map_width, map_height) {
                atenna_antinodes.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
                x -= delta_x;
                y -= delta_y;
            }
        } else {
            let mut x = (pos2.x as i32) + delta_x;
            let mut y = (pos2.y as i32) + delta_y;

            if is_valid_position(x, y, map_width, map_height) {
                atenna_antinodes.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
            }

            while all_positions && is_valid_position(x, y, map_width, map_height) {
                atenna_antinodes.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
                x += delta_x;
                y += delta_y;
            }
        }

        atenna_antinodes
    }

    fn remove_duplicates(positions: Vec<Position>) -> Vec<Position> {
        let mut unique_positions: Vec<Position> = Vec::new();
        for pos in positions.iter() {
            if !unique_positions
                .iter()
                .any(|p| p.x == pos.x && p.y == pos.y)
            {
                unique_positions.push(Position { x: pos.x, y: pos.y });
            }
        }
        unique_positions
    }

    fn get_antinodes(
        antennas: &HashMap<char, Vec<Position>>,
        map_height: usize,
        map_width: usize,
        all_positions: bool,
    ) -> Vec<Position> {
        let mut antinodes = Vec::<Position>::new();

        for (_, positions) in antennas.iter() {
            if positions.len() > 1 {
                for (pos_idx_1, pos1) in positions.iter().enumerate() {
                    for pos_idx_2 in pos_idx_1 + 1..positions.len() {
                        let pos2 = positions.get(pos_idx_2).unwrap();
                        let antenna_antinodes =
                            get_antenna_antinodes(pos1, pos2, map_width, map_height, all_positions);
                        antinodes.extend(antenna_antinodes);
                    }
                }
            }
        }

        remove_duplicates(antinodes)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_map(reader)?;
        let antennas = get_antennas(&map);
        let antinodes = get_antinodes(&antennas, map.len(), map[0].len(), false);
        Ok(antinodes.len())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_map(reader)?;
        let antennas = get_antennas(&map);
        let mut antinodes = get_antinodes(&antennas, map.len(), map[0].len(), true);

        for pos in antennas.values().flatten().collect::<Vec<&Position>>() {
            if !antinodes.iter().any(|p| p.x == pos.x && p.y == pos.y) {
                antinodes.push(Position { x: pos.x, y: pos.y });
            }
        }

        Ok(antinodes.len())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
