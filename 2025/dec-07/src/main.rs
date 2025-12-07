use std::collections::{HashSet, VecDeque};

use aoc_utils::Coord;

fn main() {
    let mut start = Coord { x: 0, y: 0 };
    let mut y = 0_usize;
    let mut splits = HashSet::<Coord>::new();
    let mut max_x = 0_usize;
    let mut full = vec![];
    aoc_utils::get_lines(&"input.txt".to_string()).for_each(|l| {
        let line = l.unwrap();
        let chars = line.chars().collect::<Vec<char>>();
        if chars.len() > max_x { max_x = chars.len(); }
        for i in 0..chars.len() {
            if chars[i] == 'S' {
                start = Coord { x: i, y };
            } else if chars[i] == '^' {
                splits.insert(Coord { x: i, y });
            }
        }

        full.push(chars);

        y += 1;
    });

    let mut p1 = p1(&splits, &start, max_x, y);
    p1.0.sort_by(|c1,c2| c1.y.cmp(&c2.y));
    println!("p1 {}", p1.0.len());
    p1.1[start.y][start.x] = '|';

    let mut track = vec![0; p1.1[0].len()];
    track[start.x] = 1;
    for split in p1.0 {
        let x = split.x;
        let val = track[x];
        track[x-1] += val;
        track[x+1] += val;
        track[x] = 0;
    }

    println!("p2: {}", track.iter().sum::<u64>());
}

fn p1(grid: &HashSet<Coord>, start: &Coord, max_x: usize, max_y: usize) -> (Vec<Coord>, Vec<Vec<char>>) {
    let mut splits = vec![];
    let mut queue = VecDeque::<Coord>::new();
    let mut path_grid = vec![vec!['.'; max_x]; max_y];
    queue.push_front(*start);


    while !queue.is_empty() {
        let mut current = queue.pop_front().unwrap();
        current.y += 1;

        if current.y >= max_y || splits.contains(&current) {
            continue;
        }

        if grid.contains(&current) {
            queue.push_front(Coord { x: current.x+1, y: current.y });
            queue.push_front(Coord { x: current.x-1, y: current.y });

            if current.y < max_y && current.x < max_x {
                path_grid[current.y][current.x+1] = '|';
                path_grid[current.y][current.x-1] = '|';
                path_grid[current.y][current.x] = '^';
            }
            splits.push(current);
        } else {
            path_grid[current.y][current.x] = '|';
            queue.push_front(current);
        }
    }

    aoc_utils::print_grid(&path_grid);
    (splits, path_grid)
}
