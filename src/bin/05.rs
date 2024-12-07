use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_rules_and_updates<R: BufRead>(
        reader: R,
    ) -> Result<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
        let mut record_updateds = false;
        let mut rules = Vec::<Vec<usize>>::new();
        let mut updates = Vec::<Vec<usize>>::new();
        for line in reader.lines() {
            let line = line?;

            if line == "" {
                record_updateds = true;
                continue;
            }

            if record_updateds {
                let update = line
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                updates.push(update);
            } else {
                let rule = line
                    .split("|")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                rules.push(rule);
            }
        }
        Ok((rules, updates))
    }

    fn check_valid_update(update: &Vec<usize>, rules: &Vec<Vec<usize>>) -> bool {
        let mut valid_update = true;
        let mut update_index_1 = 0;
        while update_index_1 < update.len() {
            for rule in rules.iter() {
                if update[update_index_1] == rule[1] {
                    let mut update_index_2 = update_index_1 + 1;
                    while update_index_2 < update.len() {
                        if update[update_index_2] == rule[0] {
                            valid_update = false;
                        }
                        update_index_2 += 1;
                    }
                }
            }
            update_index_1 += 1;
        }

        valid_update
    }

    fn fix_invalid_update(update: &mut Vec<usize>, rules: &mut Vec<Vec<usize>>) {
        let mut update_index_1 = 0;
        while update_index_1 < update.len() {
            for rule in rules.iter() {
                if update[update_index_1] == rule[1] {
                    let mut update_index_2 = update_index_1 + 1;
                    while update_index_2 < update.len() {
                        if update[update_index_2] == rule[0] {
                            let temp = update[update_index_1];
                            update[update_index_1] = update[update_index_2];
                            update[update_index_2] = temp;
                        }
                        update_index_2 += 1;
                    }
                }
            }
            update_index_1 += 1;
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (rules, updates) = parse_rules_and_updates(reader)?;
        let mut valid_score = 0;
        for update in updates {
            if check_valid_update(&update, &rules) {
                valid_score += update[(update.len() - 1) / 2];
            }
        }

        Ok(valid_score)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (mut rules, mut updates) = parse_rules_and_updates(reader)?;
        let mut valid_score = 0;

        let mut update_index = 0;

        while update_index < updates.len() {
            if !check_valid_update(&updates[update_index], &rules) {
                while !check_valid_update(&updates[update_index], &rules) {
                    fix_invalid_update(&mut updates[update_index], &mut rules);
                }
                valid_score += updates[update_index][(updates[update_index].len() - 1) / 2];
            }
            update_index += 1;
        }

        Ok(valid_score)
        // Ok(0)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
