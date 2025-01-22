use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn minimize_cost(
    a1: isize,
    b1: isize,
    c1: isize,
    a2: isize,
    b2: isize,
    c2: isize,
    cost_x: isize,
    cost_y: isize,
) -> Option<(usize, usize)> {
    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        let mut min_cost = None;
        let mut best_solution = None;

        for x in 0.. {
            let y = c1 - a1 * x;
            if y % b1 == 0 {
                let y = y / b1;
                if a2 * x + b2 * y == c2 && x >= 0 && y >= 0 {
                    let cost = cost_x * x + cost_y * y;
                    if min_cost.is_none() || cost < min_cost.unwrap() {
                        min_cost = Some(cost);
                        best_solution = Some((x as usize, y as usize));
                    }
                }
            }
            if c1 - a1 * x < 0 {
                break;
            }
        }
        return best_solution;
    }

    let x_part = (c1 * b2 - c2 * b1) / det;
    let y_part = (a1 * c2 - a2 * c1) / det;

    if (c1 * b2 - c2 * b1) % det != 0 || (a1 * c2 - a2 * c1) % det != 0 {
        return None;
    }

    if x_part < 0 || y_part < 0 {
        return None;
    }

    Some((x_part as usize, y_part as usize))
}

#[derive(Debug, Clone)]
struct EquationPair {
    a1: isize,
    b1: isize,
    c1: isize,
    a2: isize,
    b2: isize,
    c2: isize,
}

fn main() -> Result<()> {
    start_day(DAY);

    fn parse_eq<R: BufRead>(reader: R) -> Result<Vec<EquationPair>> {
        let re = Regex::new(r"\d+").unwrap();
        let mut equations = Vec::<EquationPair>::new();
        let mut index = 0;
        let mut current_equation = EquationPair {
            a1: 0,
            b1: 0,
            c1: 0,
            a2: 0,
            b2: 0,
            c2: 0,
        };
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                equations.push(EquationPair {
                    a1: current_equation.a1,
                    b1: current_equation.b1,
                    c1: current_equation.c1,
                    a2: current_equation.a2,
                    b2: current_equation.b2,
                    c2: current_equation.c2,
                });
            } else {
                let matches: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();
                if matches.len() == 2 {
                    if index % 4 == 0 {
                        current_equation.a1 = matches[0].parse()?;
                        current_equation.a2 = matches[1].parse()?;
                    } else if index % 4 == 1 {
                        current_equation.b1 = matches[0].parse()?;
                        current_equation.b2 = matches[1].parse()?;
                    } else if index % 4 == 2 {
                        current_equation.c1 = matches[0].parse()?;
                        current_equation.c2 = matches[1].parse()?;
                    }
                }
            }

            index += 1;
        }

        equations.push(current_equation);

        Ok(equations)
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let equations = parse_eq(reader)?;
        let mut token_cont = 0;
        for eq in equations {
            if let Some((x, y)) = minimize_cost(eq.a1, eq.b1, eq.c1, eq.a2, eq.b2, eq.c2, 3, 1) {
                if x <= 100 && y <= 100 {
                    token_cont += x * 3;
                    token_cont += y;
                }
            }
        }
        Ok(token_cont)
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut equations = parse_eq(reader)?;

        equations = equations
            .iter()
            .map(|eq| {
                let mut temp_eq = eq.clone();
                temp_eq.c1 += 10000000000000;
                temp_eq.c2 += 10000000000000;
                temp_eq
            })
            .collect();

        let mut token_cont = 0;
        for eq in equations {
            if let Some((x, y)) = minimize_cost(eq.a1, eq.b1, eq.c1, eq.a2, eq.b2, eq.c2, 3, 1) {
                token_cont += x * 3;
                token_cont += y;
            }
        }
        Ok(token_cont)
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
