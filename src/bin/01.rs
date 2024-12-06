use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // -------------
        let mut total_distance = 0;
        let mut left_col = Vec::new();
        let mut right_col = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<&str> = line.split_whitespace().collect();

            if numbers.len() == 2 {
                let left: i32 = numbers[0].parse()?;
                let right: i32 = numbers[1].parse()?;
                left_col.push(left);
                right_col.push(right);
            }
        }

        left_col.sort();
        right_col.sort();

        for i in 0..left_col.len() {
            let mut distance = left_col[i] - right_col[i];
            distance = distance.abs();
            total_distance += distance;
        }

        Ok(total_distance)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut total_similarity_score = 0;

        let mut left_col = Vec::new();
        let mut right_col = Vec::new();
        let mut count_map: HashMap<i32, usize> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<&str> = line.split_whitespace().collect();

            if numbers.len() == 2 {
                let left: i32 = numbers[0].parse()?;
                let right: i32 = numbers[1].parse()?;
                left_col.push(left);
                right_col.push(right);

                *count_map.entry(right).or_insert(0) += 1;
            }
        }

        for &left in &left_col {
            let count = *count_map.get(&left).unwrap_or(&0) as i32;
            total_similarity_score += left * count;
        }

        Ok(total_similarity_score)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
