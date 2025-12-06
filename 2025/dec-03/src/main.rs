
fn main() {
    let lines: Vec<Vec<char>> = aoc_utils::get_lines(&"input.txt".to_string()).map(|l| {
        l.unwrap().chars().collect()
    }).collect();
    println!("p1: {}", p1(lines.clone(), 2_usize));
    println!("p2: {}", p1(lines, 12_usize));
}

fn get_max(line: &Vec<char>, start: usize, end: usize) -> (char,usize) {
    let mut max = char::default();
    let mut max_idx = 0_usize;

    (start..end).for_each(|i| {
        if line[i] > max {
            max_idx = i;
            max = line[i];
        }
    });

    (max, max_idx)
}

fn p1(lines: Vec<Vec<char>>, len: usize) -> u64 {
    let mut total = 0_u64;
    lines.iter().for_each(|line| {
        let mut start = 0_usize;
        total += (0..len).rev().map(|i| {
            let max = get_max(&line, start, line.len()-i);
            start = max.1+1;
            max.0
        }).collect::<String>().parse::<u64>().unwrap();
    });

    total
}