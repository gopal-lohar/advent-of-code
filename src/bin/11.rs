use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_stones<R: BufRead>(reader: R) -> Result<HashMap<u128, usize>> {
        let mut stones = HashMap::<u128, usize>::new();
        for line in reader.lines() {
            let line = line?;

            if line == "" {
                continue;
            }

            let stone_line = line
                .split(" ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            for stone in stone_line {
                *stones.entry(stone).or_insert(0) += 1;
            }
        }
        Ok(stones)
    }

    fn blink(stones: HashMap<u128, usize>, blink_count: u8) -> HashMap<u128, usize> {
        let mut stones = stones;
        for _ in 0..blink_count {
            let mut updates: HashMap<u128, usize> = HashMap::new();

            for (&stone, &value) in stones.iter() {
                let digit_count = (stone as f64).log10() as u32 + 1;

                if stone == 0 {
                    *updates.entry(1).or_insert(0) += value;
                } else if digit_count % 2 == 0 {
                    let half_digits = digit_count / 2;
                    let divisor = 10_u128.pow(half_digits);
                    *updates.entry(stone / divisor).or_insert(0) += value;
                    *updates.entry(stone % divisor).or_insert(0) += value;
                } else {
                    *updates.entry(stone * 2024).or_insert(0) += value;
                }
            }

            stones = updates;
        }

        stones
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let stones = parse_stones(reader)?;
        let mut stone_count = 0;
        let stones_after_blink = blink(stones, 25);

        for (_, value) in stones_after_blink.iter() {
            stone_count += value;
        }

        Ok(stone_count)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let stones = parse_stones(reader)?;
        let mut stone_count = 0;
        let stones_after_blink = blink(stones, 75);

        for (_, value) in stones_after_blink.iter() {
            stone_count += value;
        }

        Ok(stone_count)
    }

    assert_eq!(65601038650482, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
