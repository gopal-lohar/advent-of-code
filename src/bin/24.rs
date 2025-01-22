use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "24";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

struct Equation {
    input_1: String,
    input_2: String,
    operation: String,
    result: String,
}

fn parse_values(line: String, values: &mut HashMap<String, bool>) {
    let value_vec: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
    if value_vec.len() == 2 {
        values.insert(value_vec[0].clone(), {
            if value_vec[1] == "1" {
                true
            } else {
                false
            }
        });
    }
}

fn parse_equations(line: String, equations: &mut Vec<Equation>) {
    let parts: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
    if parts.len() == 5 {
        equations.push(Equation {
            input_1: parts[0].clone(),
            operation: parts[1].clone(),
            input_2: parts[2].clone(),
            result: parts[4].clone(),
        })
    }
}

fn parse_rules_and_updates<R: BufRead>(
    reader: R,
) -> Result<(HashMap<String, bool>, Vec<Equation>)> {
    let mut get_equations = false;
    let mut values = HashMap::<String, bool>::new();
    let mut equations = Vec::<Equation>::new();

    for line in reader.lines() {
        let line = line?;
        if line == "" {
            get_equations = true;
            continue;
        }
        if !get_equations {
            parse_values(line, &mut values);
        } else {
            parse_equations(line, &mut equations);
        }
    }
    Ok((values, equations))
}

fn perform_operations(values: &mut HashMap<String, bool>, equations: &mut Vec<Equation>) {
    while equations.len() != 0 {
        let mut remove_indices = Vec::<usize>::new();
        for eq_index in 0..equations.len() {
            let eq = &equations[eq_index];
            if values.contains_key(&eq.input_1) && values.contains_key(&eq.input_2) {
                let input_1 = values.get(&eq.input_1).unwrap();
                let input_2 = values.get(&eq.input_2).unwrap();
                let result = match &eq.operation[..] {
                    "AND" => input_1 & input_2,
                    "OR" => input_1 | input_2,
                    "XOR" => input_1 ^ input_2,
                    _ => panic!("Unknown operation"),
                };
                values.insert(eq.result.clone(), result);
                remove_indices.push(eq_index);
            }
        }
        for i in remove_indices.iter().rev() {
            equations.remove(*i);
        }
    }
}

fn get_sum_by_letter(values: &HashMap<String, bool>, letter: String) -> u64 {
    let mut z_values = Vec::<String>::new();
    for (key, _) in values.iter() {
        if key[0..1] == letter {
            z_values.push(key.clone());
        }
    }
    z_values.sort();
    let mut res = 0;
    for i in 0..z_values.len() {
        if values.get(&z_values[i]).unwrap().clone() {
            res += 2_u64.pow(i as u32);
        }
    }

    res
}
fn generate_id(r: char, n: usize) -> String {
    let mut id = String::new();
    id.push(r);
    id.push_str(&format!("{:02}", n % 100)); // add a zero if shorter than a 2
    id
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut values, mut equations) = parse_rules_and_updates(reader)?;
        perform_operations(&mut values, &mut equations);
        Ok(get_sum_by_letter(&values, "z".to_string()) as usize)
    }

    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (mut values, mut equations) = parse_rules_and_updates(reader)?;
        let mut swaps: Vec<(String, String)> = Vec::new();

        for index in 0..45 {
            let xid = generate_id('x', index);
            let yid = generate_id('y', index);
            let zid = generate_id('z', index);
            if index == 0 {
                for eq in &equations {
                    if (eq.input_1 == xid || eq.input_2 == xid)
                        && (eq.input_1 == yid || eq.input_2 == yid)
                        && eq.operation == "XOR"
                    {
                        if eq.result != zid {
                            swaps.push((zid.clone(), eq.result.clone()));
                        }
                    }
                }
            } else if index == 1 {
                let mut sum1 = String::new();
                let mut carry1 = String::new();
                let mut sum2 = String::new();
                let mut carry2 = String::new();

                for eq in &equations {
                    if (eq.input_1 == xid || eq.input_2 == xid)
                        && (eq.input_1 == yid || eq.input_2 == yid)
                    {
                        if eq.operation == "XOR" {
                            println!("res: {}", eq.result);
                        } else if eq.operation == "AND" {
                            println!("res: {}", eq.result);
                        }
                    }
                }
            }
        }
        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
