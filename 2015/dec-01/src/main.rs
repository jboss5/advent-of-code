fn main() {
    let mut input = aoc_utils::get_lines(&"input.txt".to_string());
    let str = input.next().unwrap().unwrap();
    println!("p1 {}", p1(str.chars().collect()));
    println!("p2 {}", p2(str.chars().collect()));
}

fn p1(path: Vec<char>) -> i64 {
    let mut total = 0_i64;
    for ch in path {
        match ch {
            '(' => total += 1,
            ')' => total -= 1,
            _ => panic!("invalid char {}", ch),
        }
    }

    total
}

fn p2(path: Vec<char>) -> u64 {
    let mut char_idx = 0_u64;
    let mut total = 0_i64;
    for ch in path {
        match ch {
            '(' => total += 1,
            ')' => total -= 1,
            _ => panic!("invalid char {}", ch),
        }

        char_idx += 1;

        if total == -1 {
            return char_idx;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1("(())".chars().collect()), 0);
        assert_eq!(p1("()()".chars().collect()), 0);
        assert_eq!(p1("(((".chars().collect()), 3);
        assert_eq!(p1("))(((((".chars().collect()), 3);
        assert_eq!(p1("())".chars().collect()), -1);
        assert_eq!(p1("))(".chars().collect()), -1);
        assert_eq!(p1(")))".chars().collect()), -3);
        assert_eq!(p1(")())())".chars().collect()), -3);
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(")".chars().collect()), 1);
        assert_eq!(p2("()())".chars().collect()), 5);
    }
}