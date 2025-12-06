mod r#box;

use aoc_utils::{get_lines, get_lines_str, Coord};

#[derive(Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(Debug)]
struct Input {
    box_grid: Vec<Vec<char>>,
    robot_path: Vec<Direction>,
    start: Coord
}

fn print(grid: &Vec<Vec<char>>) {
    for vec in grid {
        for ch in vec {
            print!("{}", ch);
        }

        println!();
    }
}


fn main() {
    let mut input = build_input("input.txt");
    // println!("input: {:?}", input);

    part1(&mut input.box_grid, &input.robot_path, &input.start);
    // print(&input.box_grid);

    let mut count = 0;
    for h in 0..input.box_grid.len() {
        for w in 0..input.box_grid[h].len() {
            if input.box_grid[h][w] == 'O' {
                count += (100 * h) + w;
            }
        }
    }

    println!("part1: {}", count);

    // let mut input2 = build_input2("input-sample3.txt");
    // println!("{}", input2.robot_path.len());
    // print(&input2.box_grid);
    // part2(&mut input2.box_grid, &input2.robot_path, &input2.start);
    // 
    // count = 0;
    // for h in 0..input2.box_grid.len() {
    //     for w in 0..input2.box_grid[h].len() {
    //         if input2.box_grid[h][w] == '[' {
    //             count += (100 * h) + w;
    //         }
    //     }
    // }
    // 
    // println!("part2: {}", count);
}

fn part1(box_grid: &Vec<Vec<char>>, robot_path: &Vec<Direction>, start: &Coord) {
    for direction in robot_path {
        match direct
    }
}

fn build_input(file: &str) -> Input {
    let mut box_grid = vec![];
    let mut robot_path = vec![];
    let mut path = false;
    let mut start = Coord { x: 0, y: 0 };

    get_lines_str(file).for_each(|l| {
        let line = l.unwrap();
        if line.is_empty() { path = true; }
        else if(path) {
            line.chars().for_each(|ch| {
                match ch {
                    '^' => robot_path.push(Direction::NORTH),
                    'v' => robot_path.push(Direction::SOUTH),
                    '>' => robot_path.push(Direction::EAST),
                    '<' => robot_path.push(Direction::WEST),
                    _ => panic!("Invalid direction: {}", ch)
                }
            });
        } else {
            let mut ch_vec = vec![];
            let chars: Vec<char> = line.chars().collect();
            for i in 0..chars.len() {
                let ch = chars[i];
                if ch == '@' { start = Coord { y: box_grid.len(), x: i } }
                ch_vec.push(ch)
            }
            box_grid.push(ch_vec);
        }
    });

    Input { box_grid, robot_path, start }
}