use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn main() -> Result<()> {
    start_day(DAY);

    fn parse_map<R: BufRead>(reader: R) -> Result<Vec<Robot>> {
        let mut robots = Vec::<Robot>::new();
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                continue;
            }
            let re = Regex::new(r"-?\d+").unwrap();

            let first_four: Vec<isize> = re
                .captures_iter(&line)
                .take(4) // Take only the first four matches.
                .map(|cap| cap.get(0).unwrap().as_str().parse::<isize>().unwrap()) // Parse matches as isize.
                .collect();

            let robot = Robot {
                position: (first_four[0], first_four[1]),
                velocity: (first_four[2], first_four[3]),
            };

            robots.push(robot);
        }
        Ok(robots)
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, map_w: usize, map_h: usize) -> Result<usize> {
        let mut robots = parse_map(reader)?;

        let mut map = Vec::<Vec<isize>>::new();
        for _ in 0..map_h {
            let mut row = Vec::<isize>::new();
            for _ in 0..map_w {
                row.push(0);
            }
            map.push(row);
        }

        for _ in 0..100 {
            for robot_index in 0..robots.len() {
                let robot = &mut robots[robot_index];
                robot.position.0 += robot.velocity.0;
                robot.position.1 += robot.velocity.1;
                robot.position.0 =
                    ((robot.position.0 % map_w as isize) + map_w as isize) % map_w as isize;
                robot.position.1 =
                    ((robot.position.1 % map_h as isize) + map_h as isize) % map_h as isize;
            }
        }

        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        for robot in &robots {
            map[robot.position.1 as usize][robot.position.0 as usize] += 1;
            if robot.position.0 < map_w as isize / 2 && robot.position.1 < map_h as isize / 2 {
                q1 += 1;
            } else if robot.position.0 > map_w as isize / 2 && robot.position.1 < map_h as isize / 2
            {
                q2 += 1;
            } else if robot.position.0 < map_w as isize / 2 && robot.position.1 > map_h as isize / 2
            {
                q3 += 1;
            } else if robot.position.0 > map_w as isize / 2 && robot.position.1 > map_h as isize / 2
            {
                q4 += 1;
            }
        }

        // println!("q1: {}, q2: {}, q3: {}, q4: {}", q1, q2, q3, q4);
        // for row in map {
        //     for cell in row {
        //         print!(" {}", cell);
        //     }
        //     println!();
        // }

        Ok(q1 * q2 * q3 * q4)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, map_w: usize, map_h: usize) -> Result<usize> {
        let mut robots = parse_map(reader)?;

        let mut map = Vec::<Vec<isize>>::new();
        for _ in 0..map_h {
            let mut row = Vec::<isize>::new();
            for _ in 0..map_w {
                row.push(0);
            }
            map.push(row);
        }

        let mut iteration = 0;
        if map_w == 101 {
            loop {
                iteration += 1;
                for robot_index in 0..robots.len() {
                    let robot = &mut robots[robot_index];
                    robot.position.0 += robot.velocity.0;
                    robot.position.1 += robot.velocity.1;
                    robot.position.0 =
                        ((robot.position.0 % map_w as isize) + map_w as isize) % map_w as isize;
                    robot.position.1 =
                        ((robot.position.1 % map_h as isize) + map_h as isize) % map_h as isize;
                }

                for y in 0..map_h {
                    for x in 0..map_w {
                        map[y][x] = 0;
                    }
                }
                for robot in &robots {
                    map[robot.position.1 as usize][robot.position.0 as usize] += 1;
                }

                println!("ITERATION: {}", iteration);
                for row in map.clone() {
                    for cell in row {
                        if cell > 0 {
                            print!(" #");
                        } else {
                            print!(" .");
                        }
                    }
                    println!();
                }
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                print!("\x1b[2J\x1b[H");
            }
        }

        if map_w == 101 {
            Ok(iteration)
        } else {
            Ok(0)
        }
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
