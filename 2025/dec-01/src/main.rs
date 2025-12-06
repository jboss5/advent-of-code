
use aoc_utils;

fn main() {
    p1();
    p2();
}

fn p2() {
    let lines = aoc_utils::get_lines(&"input.txt".to_string());
    let mut start = 50_i32;
    let mut zero_cnt = 0_u32;

    for l in lines {
        let mut combination: String = l.unwrap();
        let direction = combination.chars().next().unwrap();
        let turn = combination.split_off(1).parse::<i32>().expect("Turn NaN");
        let orig = start;

        start = match direction {
            'L' => start-turn,
            'R' => start+turn,
            _ => panic!("Invalid direction {}", direction),
        };

        if start >= 100 || start <= -100 {
            zero_cnt += (start / 100_i32).abs() as u32;

            if (orig > 0 && start <= -100) || (orig < 0 && start >= 100) {
                zero_cnt += 1;
            }

            start = start.rem_euclid(100);
        } else if start == 0 {
            zero_cnt += 1;
        } else if (orig < 0 && start > 0) || (orig > 0 && start < 0) {
            zero_cnt += 1;
        }
    }

    println!("p2 zero: {}", zero_cnt);
}

fn p1() {
    let lines = aoc_utils::get_lines(&"input.txt".to_string());
    let mut start = 50_i32;
    let mut zero_cnt = 0_u32;

    for l in lines {
        let mut combination: String = l.unwrap();
        let direction = combination.chars().next().unwrap();
        let turn = combination.split_off(1).parse::<i32>().expect("Turn NaN");
        start = match direction {
            'L' => start-turn,
            'R' => start+turn,
            _ => panic!("Invalid direction {}", direction),
        };

        if start >= 100 || start <= -100 {
            start = start.rem_euclid(100);
        }

        if start == 0 {
            zero_cnt += 1;
        }
    }

    println!("p1 zero: {}", zero_cnt);
}