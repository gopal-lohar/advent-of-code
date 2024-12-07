use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = Vec::<Vec<char>>::new();
        for line in reader.lines() {
            let line = line?;
            let line_vec = line.chars().collect();
            lines.push(line_vec);
        }

        let mut xmas_count = 0;

        for (i, line) in lines.iter().enumerate() {
            for (j, value) in line.iter().enumerate() {
                if *value == 'X' {
                    if j < line.len() - 3 && &line[j..j + 4] == ['X', 'M', 'A', 'S'] {
                        xmas_count += 1;
                    }
                    if j > 2 && &line[j - 3..j + 1] == ['S', 'A', 'M', 'X'] {
                        xmas_count += 1;
                    }
                    if i < line.len() - 3
                        && [
                            *&lines[i][j],
                            *&lines[i + 1][j],
                            *&lines[i + 2][j],
                            *&lines[i + 3][j],
                        ] == ['X', 'M', 'A', 'S']
                    {
                        xmas_count += 1;
                    }
                    if i > 2
                        && [
                            *&lines[i - 3][j],
                            *&lines[i - 2][j],
                            *&lines[i - 1][j],
                            *&lines[i][j],
                        ] == ['S', 'A', 'M', 'X']
                    {
                        xmas_count += 1;
                    }
                    if i > 2
                        && j > 2
                        && [
                            *&lines[i - 3][j - 3],
                            *&lines[i - 2][j - 2],
                            *&lines[i - 1][j - 1],
                            *value,
                        ] == ['S', 'A', 'M', 'X']
                    {
                        xmas_count += 1;
                    }
                    if i > 2
                        && j < line.len() - 3
                        && [
                            *&lines[i - 3][j + 3],
                            *&lines[i - 2][j + 2],
                            *&lines[i - 1][j + 1],
                            *value,
                        ] == ['S', 'A', 'M', 'X']
                    {
                        xmas_count += 1;
                    }
                    if i < lines.len() - 3
                        && j > 2
                        && [
                            *value,
                            *&lines[i + 1][j - 1],
                            *&lines[i + 2][j - 2],
                            *&lines[i + 3][j - 3],
                        ] == ['X', 'M', 'A', 'S']
                    {
                        xmas_count += 1;
                    }
                    if i < lines.len() - 3
                        && j < line.len() - 3
                        && [
                            *value,
                            *&lines[i + 1][j + 1],
                            *&lines[i + 2][j + 2],
                            *&lines[i + 3][j + 3],
                        ] == ['X', 'M', 'A', 'S']
                    {
                        xmas_count += 1;
                    }
                }
            }
        }
        Ok(xmas_count)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = Vec::<Vec<char>>::new();
        for line in reader.lines() {
            let line = line?;
            let line_vec = line.chars().collect();
            lines.push(line_vec);
        }

        let mut xmas_count = 0;

        for (i, line) in lines.iter().enumerate() {
            for (j, value) in line.iter().enumerate() {
                if i > 0 && i < lines.len() - 1 && j > 0 && j < line.len() - 1 && *value == 'A' {
                    let corners_z = [
                        *&lines[i - 1][j - 1],
                        *&lines[i - 1][j + 1],
                        *&lines[i + 1][j - 1],
                        *&lines[i + 1][j + 1],
                    ];
                    if corners_z == ['M', 'S', 'M', 'S']
                        || corners_z == ['M', 'M', 'S', 'S']
                        || corners_z == ['S', 'S', 'M', 'M']
                        || corners_z == ['S', 'M', 'S', 'M']
                    {
                        xmas_count += 1;
                    }
                }
            }
        }
        Ok(xmas_count)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
