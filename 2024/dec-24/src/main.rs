use std::collections::HashMap;
use aoc_utils::get_lines_str;
use regex::Regex;

enum Operation {
    AND,
    OR,
    XOR
}

struct Equation {
    lhs_name: String,
    rhs_name: String,
    val_name: String,
    operation: Operation
}

fn main() {
    let input = build_input("input.txt");
    let mut val_map = input.0.clone();
    let eq_list = input.1;
    let mut z_list = input.2;
    z_list.sort();

    println!("{:?}", z_list);

    while !is_done(&val_map, &z_list) {
        for eq in eq_list.iter() {
            let lhs = val_map.get(&eq.lhs_name);
            let rhs = val_map.get(&eq.rhs_name);

            if lhs.is_some() && rhs.is_some() {
                let lhs_val = lhs.unwrap();
                let rhs_val = rhs.unwrap();
                let val = match eq.operation {
                    Operation::AND => lhs_val & rhs_val,
                    Operation::OR => lhs_val | rhs_val,
                    Operation::XOR => lhs_val ^ rhs_val
                };

                val_map.insert(eq.val_name.clone(), val);
            }
        }
    }

    println!("part1 {}", convert_z(&val_map, &z_list));
}

fn convert_z(val_map: &HashMap<String, usize>, z_list: &Vec<String>) -> usize {
    let mut bin_str = String::new();

    z_list.iter().for_each(|z| {
        let val = val_map.get(z).unwrap();
        bin_str = val.to_string() + &*bin_str;
    });

    println!("str: {}", bin_str);
    usize::from_str_radix(&bin_str, 2).unwrap()
}

fn is_done(val_map: &HashMap<String, usize>, z_list: &Vec<String>) -> bool {
    for z in z_list.iter() {
        if !val_map.contains_key(z) {
            return false;
        }
    }

    true
}

fn build_input(file: &str) -> (HashMap<String, usize>, Vec<Equation>, Vec<String>) {
    let mut start_map = HashMap::new();
    let mut equations = Vec::new();
    let mut z_list = Vec::new();
    let mut in_equations = false;
    let start_regex = Regex::new(r"([A-Za-z0-9]+): (\d)").unwrap();
    let eq_regex = Regex::new(r"([A-Za-z0-9]+) (AND|OR|XOR) ([A-Za-z0-9]+) -> ([A-Za-z0-9]+)").unwrap();

    get_lines_str(file).for_each(|l| {
        let line = l.unwrap();
        if line.is_empty() { in_equations = true; }
        else if !in_equations {
            let caps = start_regex.captures(&line).unwrap();
            start_map.insert(caps.get(1).unwrap().as_str().trim().to_string(), caps.get(2).unwrap().as_str().trim().to_string().parse::<usize>().unwrap());
        } else {
            let caps = eq_regex.captures(&line).unwrap();
            let val_name = caps.get(4).unwrap().as_str().trim().to_string();
            let operation = match caps.get(2).unwrap().as_str().trim() {
                "AND" => Operation::AND,
                "OR" => Operation::OR,
                "XOR" => Operation::XOR,
                _ => panic!("Invalid operation in parse {:?}", caps.get(1).unwrap())
            };

            if val_name.starts_with("z") { z_list.push(val_name.clone()); }
            equations.push(Equation {
                lhs_name: caps.get(1).unwrap().as_str().trim().to_string(),
                rhs_name: caps.get(3).unwrap().as_str().trim().to_string(),
                val_name,
                operation
            });
        }
    });

    (start_map, equations, z_list)
}