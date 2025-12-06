use std::collections::{HashMap, HashSet};
use aoc_utils::get_lines_str;
use itertools::{repeat_n, Itertools};

struct Part1 {
    designs: Vec<String>,
    cache: HashMap<String, bool>,
    cnt_cache: HashMap<String, usize>
}

impl Part1 {
    pub fn new(designs: Vec<String>) -> Part1 {
        Part1 { designs, cache: HashMap::new(), cnt_cache: HashMap::new() }
    }

    pub fn cache(&mut self, design: String, found: bool) {
        self.cache.insert(design, found);
    }

    pub fn cnt_cache(&mut self, design: String) {
        self.cnt_cache.entry(design).and_modify(|c| *c += 1).or_insert(1);
    }

    pub fn run(&mut self, design: String) -> bool {
        // println!("design {design}");

        if self.cache.contains_key(&design) {
            self.cnt_cache(design.clone());
            return self.cache[&design];
        }

        if self.designs.contains(&design) {
            self.cache(design.clone(), true);
            return true;
        }

        let mut ret = false;

        for d in self.designs.clone().iter() {
            if design.starts_with(d) {
                if self.run(design[d.len()..].to_string()) {
                    self.cache(design.clone(), true);
                    // self.cnt_cache(design.clone());
                    ret = true;
                }
            }
        }

        self.cache(design, false);
        ret
    }
}


fn main() {
    let input = build_input("input-sample.txt");
    let mut designs = input.0;
    let mut combos = input.1;
    let mut count = 0;

    let mut part1 = Part1::new(combos.clone());

    for design in designs.iter() {
        if part1.run(design.clone()) { count+=1; }
    }

    // let all_combos = (0..combos.len()).permutations(4);
    let it = (0..combos.len()).into_iter();
    // let all_combos = repeat_n(it, 4).multi_cartesian_product();
    let all_combos = it.combinations_with_replacement(6);
    println!("t: {:?}", all_combos);

    for a in all_combos {
        let mut str = String::new();
        for b in a {
            str += &*combos[b].clone();
            // str.push_str(&*b.to_string());
        }

        //0538

        println!("str {}", str);
    }

    // let mut all_combos = vec![];
    // for a in temp {
    //     let val = a.into_iter().join("");
    //     // println!("p2t {:?}", val);
    //     all_combos.push(val);
    // }

    // let mut p2_count = 0;
    // for d in designs.iter() {
    //     for p in combos.iter().permutations(d.len()).map(|v| v.into_iter().join("")) {
    //         println!("p {}", p);
    //         if p == *d {
    //             p2_count += 1;
    //             // break;
    //         }
    //     }
    // }


    println!("part1 {count}");
    // println!("part2 {:?}", part1.cache);
}

fn build_input(file: &str) -> (Vec<String>, Vec<String>) {
    let mut design_vec = vec![];
    let mut combos = vec![];
    let mut has_break = false;

    get_lines_str(file).for_each(|l| {
        let line = l.unwrap();
        if line.is_empty() {
            has_break = true;
        } else if has_break {
            design_vec.push(line);
        } else {
            combos.extend(line.split(",").map(|sp| sp.trim().to_string()).collect::<Vec<String>>());
        }
    });

    (design_vec, combos)
}
