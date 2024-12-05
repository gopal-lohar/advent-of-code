use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_levels = 0;
        for line in reader.lines(){
            let reports = line?;
            let report: Vec<&str> = reports.split_whitespace().collect();
            let mut levels: Vec<i32> = Vec::new();

            for  level in report.iter(){
                levels.push(level.parse::<i32>()?);
            }

            let mut increasing_order = false;
            let mut valid = true ;

            for (index, level) in levels.iter().enumerate(){
                if index == 0{
                    if levels[index+1]-level > 0 && levels[index+1]-level < 4{
                        increasing_order = true;
                    }else if levels[index+1]-level < 0 && levels[index+1]-level > -4{
                        increasing_order = false;
                    }else{
                        valid = false;
                        break;
                    }
                }else{
                    if increasing_order{
                        if levels[index-1]-level < 0 && levels[index-1]-level > -4{
                            continue;
                        }else{
                            valid = false;
                            break;
                        }
                    }else{
                        if levels[index-1]-level > 0 && levels[index-1]-level < 4{
                            continue;
                        }else{
                            valid = false;
                            break;
                        }
                    }
                }
            }

            if valid{
                safe_levels += 1;
            }
        }

        Ok(safe_levels)
    }

    
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

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
