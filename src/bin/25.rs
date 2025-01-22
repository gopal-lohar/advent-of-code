use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "25";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

fn parse_locks_and_keys<R: BufRead>(reader: R) -> Result<(Vec<[usize; 6]>, Vec<[usize; 6]>)> {
    let mut locks: Vec<[usize; 6]> = vec![];
    let mut keys: Vec<[usize; 6]> = vec![];

    let mut line_index = 0;
    let mut current_height = [0, 0, 0, 0, 0, 0];
    let mut is_key = true;
    for line in reader.lines() {
        let line = line?;
        let remainder = line_index % 8;
        if remainder == 0 {
            if &line[..] == "....." {
                is_key = true;
            } else if &line[..] == "#####" {
                is_key = false;
            }
        } else if remainder > 0 && remainder < 6 {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    current_height[i] += 1;
                }
            }
        } else if remainder == 6 {
            if is_key {
                // current_height[5] = (current_height[0] + 1)
                //     * (current_height[1] + 8)
                //     * (current_height[2] + 16)
                //     * (current_height[3] + 24)
                //     * (current_height[4] + 32);
                keys.push(current_height);
            } else {
                // current_height[5] = (5 - current_height[0] + 1)
                //     * (5 - current_height[1] + 8)
                //     * (5 - current_height[2] + 16)
                //     * (5 - current_height[3] + 24)
                //     * (5 - current_height[4] + 32);
                locks.push(current_height);
            }
            current_height = [0, 0, 0, 0, 0, 0];
        }

        line_index += 1;
    }

    Ok((locks, keys))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (locks, keys) = parse_locks_and_keys(reader)?;
        println!("locks: {:?}", locks);
        println!("keys: {:?}", keys);
        let mut match_count = 0;
        for lock in locks {
            for key in &keys[..] {
                let mut fit = true;
                for i in 0..lock.len() {
                    if key[i] + lock[i] > 5 {
                        fit = false;
                        break;
                    }
                }
                if fit {
                    match_count += 1;
                }
            }
        }
        Ok(match_count)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

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
