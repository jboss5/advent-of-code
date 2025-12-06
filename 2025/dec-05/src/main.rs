use std::ops::Range;

fn main() {
    let mut ranges = true;
    let mut range_list = vec![];
    let mut ingredient_list = vec![];
    aoc_utils::get_lines(&"input.txt".to_string()).for_each(|l| {
        let line = l.unwrap();
        if line.is_empty() {
            ranges = false;
        } else {
            if ranges {
                let mut split = line.split('-');
                let start = split.next().unwrap().parse::<i64>().unwrap();
                let end = split.next().unwrap().parse::<i64>().unwrap();
                range_list.push(start..end);
            } else {
                ingredient_list.push(line.parse::<i64>().unwrap());
            }
        }
    });

    // sort start to make later cmp easier
    // could be more efficient to do p2 before p1 so p1 has less to loop through
    range_list.sort_by(|r1,r2| r1.start.cmp(&r2.start));
    println!("p1 {}", p1(&range_list, &ingredient_list));
    println!("p2 {}", p2(range_list));
}

fn p1(range_list: &Vec<Range<i64>>, ingredient_list: &Vec<i64>) -> u64 {
    let mut total = 0_u64;
    for id in ingredient_list {
        for range in range_list {
            // ranges in rust are exclusive at the end
            if range.start <= *id && range.end+1 >= *id {
                total += 1;
                break;
            }
        }
    }

    total
}

fn p2(mut range_list: Vec<Range<i64>>) -> u64 {
    let mut i = 0_usize;
    while i < range_list.len()-1 {
        let r1 = &range_list[i];
        let r2 = &range_list[i+1];
        
        /*
            Use cases:
            1. r1 && r2 equal, remove r2 & keep index
            2. r2 is fully inside r1, remove r2 & keep index
            3. r2 extends r1 either r1.end+1 or < end, extend r1 end & remove r2 & keep index
            4. unrelated ranges, increment index
         */
        if r1 == r2 {
            range_list.remove(i+1);
        } else if r2.start >= r1.start && r2.end <= r1.end {
            range_list.remove(i+1);
        } else if r1.end+1 >= r2.start {
            range_list[i] = range_list[i].start..range_list[i+1].end;
            range_list.remove(i+1);
        } else {
            i += 1;
        }
    }

    // ranges in rust are exclusive at the end
    range_list.iter().map(|r| (r.end+1 - r.start) as u64).sum::<u64>()
}
