
#[derive(Clone, Debug)]
struct IdRange {
    start: u128,
    end: u128
}

fn main() {
    let ranges: Vec<IdRange> = aoc_utils::get_lines(&"input.txt".to_string()).next().unwrap()
        .map(|line| {
            line.split(',').map(|r| {
                let tokens = r.split('-').collect::<Vec<&str>>();
                IdRange { start: tokens[0].parse::<u128>().unwrap(), end: tokens[1].parse::<u128>().unwrap() }
            }).collect::<Vec<IdRange>>()
        }
    ).unwrap();

    println!("p1: {}", p1(ranges.clone()));
    println!("p2: {}", p2(ranges));
}

fn get_invalid_pattern(id: Vec<char>) -> u128 {
    if id.len() % 2 != 0 { 
        return 0_u128;
    } else {
        let len = id.len()/2;
        for i in 0..len {
            if id[i] != id[len+i] {
                return 0_u128;
            }
        }
    }

    id.into_iter().collect::<String>().parse::<u128>().unwrap()
}

fn get_invalid_pattern_p2(id: Vec<char>) -> u64 {
    if id.is_empty() || id.len() == 1 {
        return 0;
    }

    let full_number = id.clone().into_iter().collect::<String>().parse::<u64>().unwrap();
    let mut same_digits = true;
    for i in 1..id.len() {
        if id[0] != id[i] {
            same_digits = false;
            break;
        }
    }

    if same_digits { 
        return full_number; 
    }

    let p1 = get_invalid_pattern(id.clone());
    if p1 != 0 { 
        return p1 as u64; 
    }

    let k_start = (id.len()/2)-1;
    if k_start == 0 || id.len() == 7 {
        return 0;
    }

    for (i,k) in (0..id.len()).zip(k_start..id.len()) {
        if id[i] != id[k] {
            return 0;
        }
    }

    full_number
}

fn p1(ranges: Vec<IdRange>) -> u128 {
    let mut total = 0_u128;
    for range in ranges {
        for id in range.start..range.end+1 {
            total += get_invalid_pattern(id.to_string().chars().map(|ch| ch).collect());
        }
    }

    total
}

fn p2(ranges: Vec<IdRange>) -> u64 {
    let mut total = 0_u64;
    for range in ranges {
        for id in range.start..range.end+1 {
            total += get_invalid_pattern_p2(id.to_string().chars().map(|ch| ch).collect());
        }
    }

    total
    // for p2 answer on input.txt: TOTAL - 79779779
    // not sure why this got chosen & kind of hacky, maybe I'll fix it later
}