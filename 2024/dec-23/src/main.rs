use aoc_utils::get_lines_str;
use std::collections::{BTreeMap, HashSet};

fn main() {
    let input = build_input("input-sample.txt");
    let part1_count = part1(input.clone());
    let part2_count = part2(input);

    println!("part1 {:?}", part1_count);
}

fn part2(input: Vec<(String, String)>) -> i32 {
    let mut count = 0;
    let mut network_map = build_network_map(input);
    println!("part2 network map {:?}", network_map);

    // for node in network_map.keys() {
    //     let mut check = network_map.get(node).unwrap().clone();
    //     while !check.is_empty() {
    //         let i_node = check.pop().unwrap();
    //         let list = network_map.get(&i_node).unwrap();
    //         for a in list {
    //             if !check.contains(a) { check.remove(a.parse().unwrap()); }
    //         }
    //     }
    // }

    count
}

fn possible_connections(node: String, network_map: &BTreeMap<String, Vec<String>>) -> Vec<String> {
    let vals = network_map.get(&node).unwrap();
    for i in vals {
        let i_vals = network_map.get(i).unwrap();
        
    }
    
    return vec![];
}

fn part1(input: Vec<(String, String)>) -> i32 {
    let mut count = 0;
    let mut network_map = build_network_map(input);

    network_map.iter_mut().for_each(|(_k,v)| v.sort());
    let mut triples = HashSet::new();
    for k in network_map.keys() {
        let k_vals = network_map.get(k).unwrap();

        for c in network_map.get(k).unwrap() {
            for v in network_map.get(c).unwrap() {
                if k_vals.contains(v) {
                    let mut triple = vec![k, c, v];
                    triple.sort();
                    triples.insert(triple);
                }
            }
        }
    }

    for network in triples {
        for pc in network {
            if pc.starts_with(&"t") {
                count += 1;
                break;
            }
        }
    }

    count
}

fn build_network_map(input: Vec<(String, String)>) -> BTreeMap<String, Vec<String>> {
    let mut network_map = BTreeMap::new();
    input.iter().for_each(|n| {
        network_map.entry(n.0.clone()).or_insert(vec![]).push(n.1.clone());
        network_map.entry(n.1.clone()).or_insert(vec![]).push(n.0.clone());
    });

    let mut output_set = HashSet::new();
    network_map.iter().for_each(|(k,v)| {
        if k.contains("t") && v.len() == 3 {
            output_set.insert(format!("{},{}", *k, v.join(",")));
        } else if v.len() == 3 {
            for val in v {
                if val.contains("t") {
                    output_set.insert(format!("{},{}", val, *k));
                    break;
                }
            }
        }
    });

    network_map
}

fn build_input(file: &str) -> Vec<(String, String)> {
    get_lines_str(file).map(|l| {
        let line = l.unwrap();
        let mut sp = line.split("-");
        (sp.next().unwrap().to_string(), sp.next().unwrap().to_string())
    }).collect()
}