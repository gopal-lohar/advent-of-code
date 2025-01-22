use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::char;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    perimeter: HashSet<char>,
}

#[derive(Debug)]
struct Region {
    // code: char,
    perimeter: usize,
    area: usize,
}

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

    fn get_next_region(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != ' ' {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn calculate_total_price(map: &Vec<Vec<char>>, side_wise: bool) -> usize {
        let mut map = map.clone();
        let mut regions = Vec::<Region>::new();
        let mut next_region = get_next_region(&map);
        while let Some((x, y)) = next_region {
            let current_char = map[y][x];
            let mut traversed_points = Vec::<Position>::new();
            let mut current_points = vec![(x, y)];

            while !current_points.is_empty() {
                let mut new_points = Vec::<(usize, usize)>::new();
                for (x, y) in current_points.iter() {
                    if let Some(_) = traversed_points.iter().find(|p| p.x == *x && p.y == *y) {
                        continue;
                    }
                    let mut perimeter = HashSet::<char>::new();
                    if *x > 0 && map[*y][*x - 1] == current_char {
                        new_points.push((*x - 1, *y));
                    } else {
                        perimeter.insert('L');
                    }

                    if *x < map[*y].len() - 1 && map[*y][*x + 1] == current_char {
                        new_points.push((*x + 1, *y));
                    } else {
                        perimeter.insert('R');
                    }

                    if *y > 0 && map[*y - 1][*x] == current_char {
                        new_points.push((*x, *y - 1));
                    } else {
                        perimeter.insert('T');
                    }

                    if *y < map.len() - 1 && map[*y + 1][*x] == current_char {
                        new_points.push((*x, *y + 1));
                    } else {
                        perimeter.insert('B');
                    }

                    traversed_points.push(Position {
                        x: *x,
                        y: *y,
                        perimeter,
                    });
                }
                current_points = new_points;
            }

            for p in traversed_points.iter() {
                map[p.y][p.x] = ' ';
            }

            let mut total_perimeter = 0;

            if side_wise {
                {
                    let mut y = 0;
                    while y < map.len() {
                        let mut x = 0;
                        while x < map[y].len() {
                            if let Some(_) = traversed_points
                                .iter()
                                .find(|p| p.y == y && p.x == x && p.perimeter.contains(&'T'))
                            {
                                let mut temp_x = x + 1;
                                while temp_x < map[y].len() {
                                    if let Some(_) = traversed_points.iter().find(|p| {
                                        p.y == y && p.x == temp_x && p.perimeter.contains(&'T')
                                    }) {
                                        temp_x += 1;
                                    } else {
                                        break;
                                    }
                                }
                                total_perimeter += 1;
                                x = temp_x;
                            } else {
                                x += 1;
                            }
                        }

                        x = 0;
                        while x < map[y].len() {
                            if let Some(_) = traversed_points
                                .iter()
                                .find(|p| p.y == y && p.x == x && p.perimeter.contains(&'B'))
                            {
                                let mut temp_x = x + 1;
                                while temp_x < map[y].len() {
                                    if let Some(_) = traversed_points.iter().find(|p| {
                                        p.y == y && p.x == temp_x && p.perimeter.contains(&'B')
                                    }) {
                                        temp_x += 1;
                                    } else {
                                        break;
                                    }
                                }
                                total_perimeter += 1;
                                x = temp_x;
                            } else {
                                x += 1;
                            }
                        }

                        y += 1;
                    }
                }

                {
                    let mut x = 0;
                    while x < map[0].len() {
                        let mut y = 0;
                        while y < map.len() {
                            if let Some(_) = traversed_points
                                .iter()
                                .find(|p| p.y == y && p.x == x && p.perimeter.contains(&'L'))
                            {
                                let mut temp_y = y + 1;
                                while temp_y < map.len() {
                                    if let Some(_) = traversed_points.iter().find(|p| {
                                        p.y == temp_y && p.x == x && p.perimeter.contains(&'L')
                                    }) {
                                        temp_y += 1;
                                    } else {
                                        break;
                                    }
                                }
                                total_perimeter += 1;
                                y = temp_y;
                            } else {
                                y += 1;
                            }
                        }

                        y = 0;
                        while y < map.len() {
                            if let Some(_) = traversed_points
                                .iter()
                                .find(|p| p.y == y && p.x == x && p.perimeter.contains(&'R'))
                            {
                                let mut temp_y = y + 1;
                                while temp_y < map.len() {
                                    if let Some(_) = traversed_points.iter().find(|p| {
                                        p.y == temp_y && p.x == x && p.perimeter.contains(&'R')
                                    }) {
                                        temp_y += 1;
                                    } else {
                                        break;
                                    }
                                }
                                total_perimeter += 1;
                                y = temp_y;
                            } else {
                                y += 1;
                            }
                        }
                        x += 1;
                    }
                }
            } else {
                for p in traversed_points.iter() {
                    total_perimeter += p.perimeter.len();
                }
            }

            regions.push(Region {
                perimeter: total_perimeter,
                area: traversed_points.len(),
            });

            next_region = get_next_region(&map);
        }

        let mut total_price = 0;

        for r in regions.iter() {
            total_price += r.area * r.perimeter;
        }

        total_price
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_map(reader)?;
        let total_price = calculate_total_price(&map, false);
        Ok(total_price)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_map(reader)?;
        let total_price = calculate_total_price(&map, true);
        Ok(total_price)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
