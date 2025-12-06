use std::{fs::File, io::{BufReader, Lines}};
use regex::Regex;

fn main() {
    let lines = aoc_utils::get_lines(&"input.txt".to_string());
    let input = parse_math(lines);
    println!("p1 {}", p1(&input));
    println!("p2 {}", p2(&input));
}

fn parse_math(lines: Lines<BufReader<File>>) -> (Vec<Vec<String>>, Vec<String>) {
    let mut statements = vec![];
    let mut operators = vec![];
    let line_vec = lines.map(|l| l.unwrap()).collect::<Vec<String>>();

    for line in &line_vec {
        let tokens: Vec<String> = Regex::new("\\s+")
            .unwrap()
            .split(&line)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if tokens[0].chars().nth(0).unwrap().is_numeric() {
            statements.push(tokens);
        } else {
            operators.extend(tokens);
        }
    }

    let mut max_col = vec![0_usize; statements[0].len()];
    for i in 0..statements.len() {
        for k in 0..statements[i].len() {
            if statements[i][k].len() > max_col[k] {
                max_col[k] = statements[i][k].len();
            }
        }
    }

    let mut output = vec![vec![]; statements[0].len()];
    for line in &line_vec {
        if line.starts_with("+") || line.starts_with("*") {
            break;
        }

        let chars = line.chars().collect::<Vec<char>>();
        let mut col = 0_usize;
        let mut i = 0;
        while i < chars.len() {
            let max = max_col[col];
            output[col].push((i..i+max).map(|k| chars[k]).collect::<String>());
            i = i+max+1;
            col += 1;
        }
    }

    (output, operators)
}

fn p1(input: &(Vec<Vec<String>>, Vec<String>)) -> i64 {
    let mut total = 0_i64;
    for i in 0..input.1.len() {
        total += match input.1[i].as_str() {
            "+" => input.0[i].iter().map(|v| v.trim().parse::<i64>().unwrap()).sum::<i64>(),
            "*" => input.0[i].iter().map(|v| v.trim().parse::<i64>().unwrap()).product::<i64>(),
            _ => panic!("unsupported operator {}", input.1[i]),
        };
    }

    total
}

fn p2(input: &(Vec<Vec<String>>, Vec<String>)) -> i64 {
    let mut total = 0_i64;
    for i in (0..input.1.len()).rev() {
        let numbers = &input.0[i];
        let mut col_nums = vec![];
        for i in (0..numbers[0].len()).rev() {
            col_nums.push(
                (0..numbers.len()).filter_map(|k| {
                    let ch = numbers[k].chars().nth(i).unwrap();
                    if ch != ' ' {
                        Some(ch)
                    } else {
                        None
                    }
                }
            ).collect::<String>().parse::<i64>().unwrap());
        }

        total += match input.1[i].as_str() {
            "+" => col_nums.iter().sum::<i64>(),
            "*" => col_nums.iter().product::<i64>(),
            _ => panic!("unsupported operator {}", input.1[i]),
        };
    }
    
    total
}
