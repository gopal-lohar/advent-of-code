use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_calib_equations<R: BufRead>(reader: R) -> Result<Vec<Vec<i128>>> {
        let mut calib_equations = Vec::<Vec<i128>>::new();
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                continue;
            }

            let parts: Vec<&str> = line.split(':').collect();
            let mut calib_equation: Vec<i128> = Vec::new();
            calib_equation.push(parts[0].trim().parse::<i128>().unwrap());
            calib_equation.extend(
                parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i128>().unwrap()),
            );
            calib_equations.push(calib_equation);
        }
        Ok(calib_equations)
    }

    fn generate_operator_vector(
        operators: &Vec<char>,
        mut seed: usize,
        position_count: u32,
    ) -> Vec<char> {
        let operator_count = operators.len();
        let mut operator_vector = Vec::<char>::new();

        while seed > 0 {
            operator_vector.push(operators[seed % operator_count]);
            seed /= operator_count;
        }

        while operator_vector.len() < position_count as usize {
            operator_vector.push(operators[0]);
        }

        operator_vector.reverse();
        operator_vector
    }

    fn verify_calib_equation(equation: &Vec<i128>, operators: &Vec<char>) -> bool {
        let answer = equation[0];
        let position_count = (equation.len() - 2) as u32;
        let operator_count = operators.len();
        let mut index = 0;
        while index < operator_count.pow(position_count) {
            let mut calculated_answer = equation[1];
            let operator_vector = generate_operator_vector(operators, index, position_count);
            for (operator_index, operator) in operator_vector.iter().enumerate() {
                if *operator == '+' {
                    calculated_answer += equation[operator_index + 2]
                } else if *operator == '*' {
                    calculated_answer *= equation[operator_index + 2]
                } else if *operator == '|' {
                    let mut multipllier = 1;
                    let mut next_num = equation[operator_index + 2];
                    while next_num > 0 {
                        multipllier *= 10;
                        next_num /= 10;
                    }
                    calculated_answer =
                        (calculated_answer * multipllier) + equation[operator_index + 2];
                }
            }
            if calculated_answer == answer {
                return true;
            }
            index += 1;
        }
        return false;
    }

    fn part1<R: BufRead>(reader: R) -> Result<i128> {
        let calib_equations = parse_calib_equations(reader)?;
        let mut answer = 0 as i128;
        for calib_equation in calib_equations {
            if verify_calib_equation(&calib_equation, &vec!['+', '*']) {
                answer += calib_equation[0];
            }
        }

        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i128> {
        let calib_equations = parse_calib_equations(reader)?;
        let mut answer = 0 as i128;
        for calib_equation in calib_equations {
            if verify_calib_equation(&calib_equation, &vec!['+', '*', '|']) {
                answer += calib_equation[0];
            }
        }

        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
