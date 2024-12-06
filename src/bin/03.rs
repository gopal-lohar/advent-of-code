use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\
";

struct Mul {
    ans: usize,
    start_idx: usize,
}

struct DoNotMul {
    ans: String,
    start_idx: usize,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn get_answers(reports: &str) -> Vec<Mul> {
        let mut answers: Vec<Mul> = Vec::new();
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        for caps in re.captures_iter(reports) {
            let m = caps.get(0).unwrap();
            let a = &caps[1].parse::<i32>().unwrap(); // First capture group
            let b = &caps[2].parse::<i32>().unwrap(); // Second capture group
            let start = m.start();
            answers.push(Mul {
                ans: (a * b) as usize,
                start_idx: start,
            });
        }
        answers
    }

    fn get_do_donts(reports: &str) -> Vec<DoNotMul> {
        let mut answers: Vec<DoNotMul> = Vec::new();
        let re = Regex::new(r"do(n't)?\(\)").unwrap();
        for cap in re.captures_iter(reports) {
            let m = cap.get(0).unwrap();
            let start = m.start();
            if m.as_str() == "don't()" {
                answers.push(DoNotMul {
                    ans: "dont".to_string(),
                    start_idx: start,
                });
            } else if m.as_str() == "do()" {
                answers.push(DoNotMul {
                    ans: "do".to_string(),
                    start_idx: start,
                });
            }
        }
        answers
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answers: Vec<Mul> = Vec::new();
        let mut sum = 0;
        for line in reader.lines() {
            let reports = line?;
            let answers1 = get_answers(&reports);
            answers.extend(answers1);
        }
        for a in answers {
            sum += a.ans;
        }
        Ok(sum as usize)
    }

    assert_eq!(161, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut sum = 0;
        let mut corrupted_mem = "".to_string();

        for line in reader.lines() {
            corrupted_mem += &line?;
        }

        let answers: Vec<Mul> = get_answers(&corrupted_mem);
        let do_donts: Vec<DoNotMul> = get_do_donts(&corrupted_mem);

        for a in answers {
            if let Some(d) = do_donts.iter().rfind(|x| x.start_idx < a.start_idx) {
                if d.ans == "do" {
                    sum += a.ans;
                }
            } else {
                sum += a.ans;
            }
        }
        Ok(sum as usize)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
