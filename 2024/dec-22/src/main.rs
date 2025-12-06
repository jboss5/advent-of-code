use std::collections::{HashMap, HashSet};
use std::ops::BitXor;
use aoc_utils::get_lines_str;

#[derive(Eq, Hash, PartialEq)]
struct Iteration {
    number: usize,
    start: u64
}

struct SecretFinder {
    cache: HashMap<u64, u64>
}

impl SecretFinder {
    fn new() -> SecretFinder {
        SecretFinder { cache: HashMap::new() }
    }

    pub fn mix(secret: &u64, num: &u64) -> u64 {
        secret.bitxor(num)
    }

    pub fn prune(num: &u64) -> u64 {
        num % 16777216
    }

    pub fn compute(&mut self, start: u64, iterations: usize) -> (u64, Vec<i64>, Vec<i64>) {
        let mut final_num = start;
        let mut diff_list: Vec<i64> = vec![];
        let mut total_list: Vec<i64> = vec![];

        for i in 0..iterations {
            let current = final_num;
            let it = Iteration { number: i, start: final_num };
            if self.cache.contains_key(&current) {
                final_num = self.cache[&current];
            } else {
                let mut next = final_num * 64;
                next = SecretFinder::mix(&final_num, &next);
                final_num = SecretFinder::prune(&next);

                next = final_num / 32;
                next = SecretFinder::mix(&final_num, &next);
                final_num = SecretFinder::prune(&next);

                next = final_num * 2048;
                next = SecretFinder::mix(&final_num, &next);
                final_num = SecretFinder::prune(&next);

                self.cache.insert(current, final_num);
            }

            let current_digit = (final_num % 10) as i64;
            // if i > 0 {
                let diff = current_digit - (current % 10) as i64;
                diff_list.push(diff);
                // println!("{}: {} {}", final_num, current_digit, diff);
            // }

            total_list.push(current_digit);
        }

        (final_num,diff_list,total_list)
    }
}

fn main() {
    let secrets = build_input("input-sample2.txt");
    println!("starting secrets: {secrets:?}");
    let mut secret_finder = SecretFinder::new();
    let mut max_map: HashMap<&u64, (Vec<i64>,Vec<i64>)> = HashMap::new();
    // let mut cache = HashSet::new();

    let sum = secrets.iter().map(|s| {
        let val = secret_finder.compute(*s, 2000);
        max_map.insert(s, (val.1.clone(), val.2.clone()));
        val.0
    }).sum::<u64>();

    println!("part1 sum: {sum}");

    let mut sum_list = vec![];
    for e in max_map.iter() {
        let mut e_i = 0usize;
        while e_i < e.1.1.len()-5 {
            let mut run_diff_sum = e.1.1[e_i];
            for n in max_map.iter() {
                if n.0 == e.0 { continue; }

                let mut n_i = 0usize;
                while n_i < n.1.1.len()-5 {
                    let val = (n.0, n.1.1[n_i]);
                    // if !cache.contains(&val) && e.1.0[e_i] == n.1.0[n_i] && e.1.0[e_i+1] == n.1.0[n_i+1] && e.1.0[e_i+2] == n.1.0[n_i+2] && e.1.0[e_i+3] == n.1.0[n_i+3] {
                    if e.1.0[e_i] == n.1.0[n_i] && e.1.0[e_i+1] == n.1.0[n_i+1] && e.1.0[e_i+2] == n.1.0[n_i+2] && e.1.0[e_i+3] == n.1.0[n_i+3] {
                        println!("FOUND e {} [{}]: {} {} val {} ch {} {} {} {} [{}] {:?}", e.0, e_i, e.1.1[e_i], n.0, n.1.1[n_i], e.1.0[e_i], e.1.0[e_i+1], e.1.0[e_i+2], e.1.0[e_i+3], n_i, n.1.1);
                        run_diff_sum += val.1;
                        // cache.insert(val);
                    }

                    n_i += 1;
                }
            }

            sum_list.push(run_diff_sum);

            e_i += 1;
        }
    }

    // println!("p2p2 {:?}", sum_list);
    println!("part2 sum: {}", sum_list.iter().max().unwrap());
}

fn build_input(file: &str) -> Vec<u64> {
    get_lines_str(file).map(|l|
        l.unwrap().parse::<u64>().unwrap()
    ).collect()
}