use std::collections::HashSet;

use aoc_utils::coord::Coord;

fn main() {
    let path = aoc_utils::get_lines_str("input.txt")
        .map(|l| l.unwrap().to_string())
        .collect::<Vec<String>>();

    println!("p1: {}", p1(path[0].clone()).len());

    let paths = split_path(path[0].clone());
    println!("p2: {}", join_paths(p1(paths.0), p1(paths.1)).len());
}

fn p1(path: String) -> HashSet<Coord> {
    let mut current = Coord { x: 0, y: 0 };
    let mut uniq_coords = HashSet::new();
    uniq_coords.insert(current.clone());

    path.chars().for_each(|ch| {
        match ch {
            '>' => current.x += 1,
            'v' => current.y += 1,
            '<' => current.x -= 1,
            '^' => current.y -= 1,
            _ => panic!("Invalid state found"),
        }

        uniq_coords.insert(current.clone());
    });

    uniq_coords
}

fn join_paths(p1: HashSet<Coord>, p2: HashSet<Coord>) -> HashSet<Coord> {
    let mut joined = HashSet::new();
    joined.extend(p1);
    joined.extend(p2);
    joined
}

fn split_path(path: String) -> (String, String) {
    let mut p1 = String::new();
    let mut p2 = String::new();
    let chars = path.chars().collect::<Vec<char>>();

    (0..chars.len() - 1).step_by(2).for_each(|i| {
        p1.push(chars[i]);
        p2.push(chars[i + 1]);
    });

    (p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_s1_test() {
        let str = ">".to_string();
        assert_eq!(p1(str).len(), 2);
    }

    #[test]
    fn p1_s2_test() {
        let str = "^>v<".to_string();
        assert_eq!(p1(str).len(), 4);
    }

    #[test]
    fn p1_s3_test() {
        let str = "^v^v^v^v^v".to_string();
        assert_eq!(p1(str).len(), 2);
    }

    #[test]
    fn split_test() {
        let paths = split_path("^v^v^v^v^v".to_string());
        assert_eq!("^^^^^", paths.0);
        assert_eq!("vvvvv", paths.1);
    }

    #[test]
    fn p2_s1_test() {
        let paths = split_path("^v".to_string());
        assert_eq!(join_paths(p1(paths.0), p1(paths.1)).len(), 3);
    }

    #[test]
    fn p2_s2_test() {
        let paths = split_path("^>v<".to_string());
        assert_eq!(join_paths(p1(paths.0), p1(paths.1)).len(), 3);
    }

    #[test]
    fn p2_s3_test() {
        let paths = split_path("^v^v^v^v^v".to_string());
        assert_eq!(join_paths(p1(paths.0), p1(paths.1)).len(), 11);
    }
}
