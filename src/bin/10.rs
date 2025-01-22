use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    fn parse_map<R: BufRead>(reader: R) -> Result<Vec<Vec<usize>>> {
        let mut map = Vec::<Vec<usize>>::new();
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                continue;
            }
            let mut map_line = Vec::<usize>::new();
            for c in line.chars() {
                map_line.push(c.to_digit(10).unwrap() as usize);
            }
            map.push(map_line);
        }
        Ok(map)
    }

    fn calculate_trailhead_score(
        x: usize,
        y: usize,
        map: &Vec<Vec<usize>>,
        ratings: bool,
    ) -> usize {
        let mut score = 0;
        let mut depth = 0;
        let mut valid_trailpoints = vec![(x, y)];
        while depth < 9 {
            let mut new_valid_trailpoints = Vec::<(usize, usize)>::new();
            for (x, y) in valid_trailpoints.iter() {
                if *y > 0 && map[(*y as usize) - 1][*x] == depth + 1 {
                    new_valid_trailpoints.push((*x, *y - 1));
                }
                if *x < map[*y].len() - 1 && map[*y][*x + 1] == depth + 1 {
                    new_valid_trailpoints.push((*x + 1, *y));
                }
                if *y < map.len() - 1 && map[*y + 1][*x] == depth + 1 {
                    new_valid_trailpoints.push((*x, *y + 1));
                }
                if *x > 0 && map[*y][*x - 1] == depth + 1 {
                    new_valid_trailpoints.push((*x - 1, *y));
                }
            }

            valid_trailpoints = new_valid_trailpoints;

            if valid_trailpoints.len() == 0 {
                break;
            } else {
                depth += 1;
            }
        }

        if depth == 9 {
            if ratings {
                score = valid_trailpoints.len();
            } else {
                let unique: HashSet<(usize, usize)> = valid_trailpoints.into_iter().collect();
                score = unique.len();
            }
        }
        score
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_map(reader)?;
        let mut total_trailhead_score = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == 0 {
                    total_trailhead_score += calculate_trailhead_score(x, y, &map, false);
                }
            }
        }
        Ok(total_trailhead_score as usize)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = parse_map(reader)?;
        let mut total_trailhead_score = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == 0 {
                    total_trailhead_score += calculate_trailhead_score(x, y, &map, true);
                }
            }
        }
        Ok(total_trailhead_score as usize)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
