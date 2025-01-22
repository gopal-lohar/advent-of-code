use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_disk<R: BufRead>(reader: R) -> Result<Vec<char>> {
        let mut disk = Vec::<char>::new();
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                continue;
            }
            disk.extend(line.chars());
        }
        Ok(disk)
    }

    fn convert_to_blocks(disk: &Vec<char>) -> Vec<String> {
        let mut blocks = Vec::<String>::new();
        for (index, c) in disk.iter().enumerate() {
            let count = c.to_digit(10).unwrap() as usize;
            for _ in 0..count {
                if index % 2 == 0 {
                    blocks.push((index / 2).to_string());
                } else {
                    blocks.push(".".to_string());
                }
            }
        }
        blocks
    }

    fn move_file_blocks(blocks: &Vec<String>) -> Vec<String> {
        let mut organised_disk = blocks.clone();
        let mut left_index = 0;
        let mut right_index = blocks.len() - 1;
        while left_index < right_index {
            if blocks[left_index] != ".".to_string() {
                left_index += 1;
            } else if blocks[right_index] == ".".to_string() {
                right_index -= 1;
            } else {
                organised_disk[left_index] = blocks[right_index].clone();
                organised_disk[right_index] = ".".to_string();
                left_index += 1;
                right_index -= 1;
            }
        }
        organised_disk
    }

    // failed attempt
    // fn move_files(blocks: &Vec<String>, disk_map: &Vec<char>) -> Vec<String> {
    //     let mut organised_disk = blocks.clone();
    //     let mut disk_map = disk_map.clone();
    //     let mut file_index = disk_map.len() - 1;
    //     'file_loop: while file_index > 0 {
    //         if file_index % 2 == 0 {
    //             for empty_space_index in 0..file_index {
    //                 if empty_space_index % 2 != 0 {
    //                     let empty_space: i32 =
    //                         disk_map[empty_space_index].to_string().parse().unwrap();
    //                     let file_size: i32 = disk_map[file_index].to_string().parse().unwrap();
    //                     if empty_space >= file_size {
    //                         // Move the file in disk_map
    //                         let temp_disk_map = disk_map.clone();
    //                         disk_map.insert(empty_space_index, '0');
    //                         disk_map[empty_space_index + 1] =
    //                             file_size.to_string().chars().next().unwrap();
    //                         disk_map.insert(
    //                             empty_space_index + 2,
    //                             (empty_space - file_size)
    //                                 .to_string()
    //                                 .chars()
    //                                 .next()
    //                                 .unwrap(),
    //                         );
    //                         disk_map[file_index] = '0';

    //                         // Move the file in organised_disk
    //                         let mut empty_space_index_in_blocks = 0;
    //                         for count_index in 0..(empty_space_index) {
    //                             empty_space_index_in_blocks += temp_disk_map[count_index]
    //                                 .to_string()
    //                                 .parse::<i32>()
    //                                 .unwrap();
    //                         }

    //                         let mut file_index_in_blocks = 0;
    //                         for count_index in 0..(file_index) {
    //                             file_index_in_blocks += temp_disk_map[count_index]
    //                                 .to_string()
    //                                 .parse::<i32>()
    //                                 .unwrap();
    //                         }

    //                         // println!(
    //                         //     "empty_space_index_in_blocks: {}",
    //                         //     empty_space_index_in_blocks
    //                         // );
    //                         // println!("file_index_in_block: {}", file_index_in_blocks);

    //                         for i in 0..file_size {
    //                             organised_disk[empty_space_index_in_blocks as usize + i as usize] =
    //                                 organised_disk[file_index_in_blocks as usize + i as usize]
    //                                     .clone();
    //                             organised_disk[file_index_in_blocks as usize + i as usize] =
    //                                 ".".to_string();
    //                         }
    //                         file_index -= 1;
    //                         continue 'file_loop;
    //                     }
    //                 }
    //             }
    //         }
    //         file_index -= 1;
    //     }

    //     organised_disk
    // }

    fn move_files(blocks: &Vec<String>) -> Vec<String> {
        let mut organised_disk = blocks.clone();
        let mut file_index = blocks.len() - 1;
        while file_index > 0 {
            if organised_disk[file_index] == ".".to_string() {
                file_index -= 1;
                continue;
            }
            let file_id = organised_disk[file_index].clone();

            let mut temp_file_index = file_index;
            while temp_file_index > 0 && organised_disk[temp_file_index] == file_id {
                temp_file_index -= 1;
            }
            let file_size = file_index - temp_file_index;

            let empty_space_index;
            for i in 0..file_index {
                if organised_disk[i] == ".".to_string() {
                    let mut temp_empty_space_index = i;
                    while organised_disk[temp_empty_space_index] == ".".to_string() {
                        temp_empty_space_index += 1;
                    }
                    if temp_empty_space_index - i >= file_size {
                        empty_space_index = i;
                        for i in 0..file_size {
                            organised_disk[empty_space_index + i] =
                                organised_disk[temp_file_index + 1 + i].clone();
                            organised_disk[temp_file_index + 1 + i] = ".".to_string();
                        }
                        break;
                    }
                }
            }
            file_index -= file_size;
        }
        organised_disk
    }

    fn calculate_checksum(disk: &Vec<String>) -> usize {
        let mut checksum = 0;
        for (index, c) in disk.iter().enumerate() {
            if *c == ".".to_string() {
                continue;
            }
            let digit: i32 = c.parse().unwrap();
            checksum += index * digit as usize
        }
        checksum
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let disk = parse_disk(reader)?;
        let blocks = convert_to_blocks(&disk);
        let organised_disk = move_file_blocks(&blocks);
        let checksum = calculate_checksum(&organised_disk);
        Ok(checksum)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let disk = parse_disk(reader)?;
        let blocks = convert_to_blocks(&disk);
        let organised_disk = move_files(&blocks);
        let checksum = calculate_checksum(&organised_disk);
        Ok(checksum)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
