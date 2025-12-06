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

fn find_start(grid: &Vec<Vec<char>>) -> Coord {
    let mut start = Coord { x: 0, y: 0 };
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                return Coord { x, y };
            }
        }
    }

    start
}


fn main() {
    let mut input = build_input("input.txt");
    // println!("input: {:?}", input);

    // part1(&mut input.box_grid, &input.robot_path, &input.start);
    // print(&input.box_grid);
    //
    let mut count = 0;
    // for h in 0..input.box_grid.len() {
    //     for w in 0..input.box_grid[h].len() {
    //         if input.box_grid[h][w] == 'O' {
    //             count += (100 * h) + w;
    //         }
    //     }
    // }
    //
    // println!("part1: {}", count);

    let mut input2 = build_input2("input-sample3.txt");
    // println!("{}", input2.robot_path.len());
    let start = find_start(&input2.box_grid);
    print(&input2.box_grid);
    part2(&mut input2.box_grid, &input2.robot_path, &start);

    count = 0;
    for h in 0..input2.box_grid.len() {
        for w in 0..input2.box_grid[h].len() {
            if input2.box_grid[h][w] == '[' {
                count += (100 * h) + w;
            }
        }
    }
    
    println!("part2: {}", count);
}

fn part2(box_grid: &mut Vec<Vec<char>>, robot_path: &Vec<Direction>, start: &Coord) {
    let mut current = start.clone();
    println!("start {:?}", current);
    for direction in robot_path {
        match direction {
            Direction::EAST => {
                match box_grid[current.y][current.x+1] {
                    '['|']' => {
                        let mut ch = &box_grid[current.y][current.x+1];
                        let mut x = current.x+1;
                        while ['[',']'].contains(&box_grid[current.y][x]) && x < box_grid[current.y].len()-1 {
                            x += 1;
                            ch = &box_grid[current.y][x];
                        }

                        if ch != &'#' {
                            box_grid[current.y][current.x] = '.';
                            current.x = current.x+1;
                            box_grid[current.y][current.x] = '@';
                            (current.x+1..x+1).step_by(2).for_each(|n_x| {
                                box_grid[current.y][n_x] = '[';
                                box_grid[current.y][n_x+1] = ']';
                            });
                        }
                    },
                    '#' => {}, // do nothing
                    _ => {
                        box_grid[current.y][current.x] = '.';
                        current.x = current.x+1;
                        box_grid[current.y][current.x] = '@';
                    }
                }
            },
            Direction::WEST => {
                match box_grid[current.y][current.x-1] {
                    '['|']' => {
                        let mut ch = &box_grid[current.y][current.x-1];
                        let mut x = current.x-1;
                        while ['[',']'].contains(&box_grid[current.y][x]) && x > 0 {
                            x -= 1;
                            ch = &box_grid[current.y][x];
                        }

                        if ch != &'#' {
                            box_grid[current.y][current.x] = '.';
                            current.x = current.x-1;
                            box_grid[current.y][current.x] = '@';
                            (x..current.x).step_by(2).for_each(|n_x| {
                                box_grid[current.y][n_x] = '[';
                                box_grid[current.y][n_x+1] = ']';
                            });
                        }
                    },
                    '#' => {}, // do nothing
                    _ => {
                        box_grid[current.y][current.x] = '.';
                        current.x = current.x-1;
                        box_grid[current.y][current.x] = '@';
                    }
                }
            },
            Direction::NORTH => {
                match box_grid[current.y-1][current.x] {
                    '#' => {}, // do nothing
                    '.' => {
                        box_grid[current.y][current.x] = '.';
                        current.y = current.y-1;
                        box_grid[current.y][current.x] = '@';
                    },
                    ']' => {
                        let mut ch = &']';
                        let mut y = current.y-1;
                        let mut x = current.x;
                        // let mut moves = vec![];
                        while !['#','.'].contains(&box_grid[y][x]) && y > 0 {
                            match ch {
                                '[' => {
                                    y -= 1;
                                    x += 1;
                                },
                                ']' => {
                                    y -= 1;
                                },
                                _ => {}
                            }

                            ch = &box_grid[y][x];
                        }

                        if ch != &'#' {
                            box_grid[current.y][current.x] = '.';
                            current.y = current.y-1;
                            box_grid[current.y][current.x] = '@';
                            (y..current.y).for_each(|n_y| box_grid[n_y][current.x] = 'O');
                        }
                    },
                    _ => {}
                }
            },
            Direction::SOUTH => {
                match box_grid[current.y+1][current.x] {
                    '#' => {}, // do nothing
                    '.' => {
                        box_grid[current.y][current.x] = '.';
                        current.y = current.y+1;
                        box_grid[current.y][current.x] = '@';
                    },
                    _ => {}
                }
            }
        }

        print(&box_grid);
    }
}

fn part1(box_grid: &mut Vec<Vec<char>>, robot_path: &Vec<Direction>, start: &Coord) {
    let mut current = start.clone();
    for direction in robot_path {
        match direction {
            Direction::NORTH => {
                match box_grid[current.y-1][current.x] {
                    'O' => {
                        let mut ch = &'O';
                        let mut y = current.y-1;
                        while ch == &'O' && y > 0 {
                            y -= 1;
                            ch = &box_grid[y][current.x];
                        }

                        if ch != &'#' {
                            box_grid[current.y][current.x] = '.';
                            current.y = current.y-1;
                            box_grid[current.y][current.x] = '@';
                            (y..current.y).for_each(|n_y| box_grid[n_y][current.x] = 'O');
                        }
                    },
                    '#' => {}, // do nothing
                    _ => {
                        box_grid[current.y][current.x] = '.';
                        current.y = current.y-1;
                        box_grid[current.y][current.x] = '@';
                    }
                }
            },
            Direction::SOUTH => {
                match box_grid[current.y+1][current.x] {
                    'O' => {
                        let mut ch = &'O';
                        let mut y = current.y+1;
                        while ch == &'O' && y < box_grid.len()-1 {
                            y += 1;
                            ch = &box_grid[y][current.x];
                        }

                        if ch != &'#' {
                            box_grid[current.y][current.x] = '.';
                            current.y = current.y + 1;
                            box_grid[current.y][current.x] = '@';
                            (current.y+1..y+1).for_each(|n_y| box_grid[n_y][current.x] = 'O');
                        }
                    },
                    '#' => {}, // do nothing
                    _ => {
                        box_grid[current.y][current.x] = '.';
                        current.y = current.y+1;
                        box_grid[current.y][current.x] = '@';
                    }
                }
            },
            Direction::EAST => {
                match box_grid[current.y][current.x+1] {
                    'O' => {
                        let mut ch = &'O';
                        let mut x = current.x+1;
                        while &box_grid[current.y][x] == &'O' && x < box_grid[current.y].len()-1 {
                            x += 1;
                            ch = &box_grid[current.y][x];
                        }

                        if ch != &'#' {
                            box_grid[current.y][current.x] = '.';
                            current.x = current.x+1;
                            box_grid[current.y][current.x] = '@';
                            (current.x+1..x+1).for_each(|n_x| box_grid[current.y][n_x] = 'O');
                        }
                    },
                    '#' => {}, // do nothing
                    _ => {
                        box_grid[current.y][current.x] = '.';
                        current.x = current.x+1;
                        box_grid[current.y][current.x] = '@';
                    }
                }
            },
            Direction::WEST => {
                match box_grid[current.y][current.x-1] {
                    'O' => {
                        let mut ch = &'O';
                        let mut x = current.x-1;
                        while ch == &'O' && x > 0 {
                            x -= 1;
                            ch = &box_grid[current.y][x];
                        }

                        if ch != &'#' {
                            box_grid[current.y][current.x] = '.';
                            current.x = current.x-1;
                            box_grid[current.y][current.x] = '@';
                            (x..current.x).for_each(|n_x| box_grid[current.y][n_x] = 'O');
                        }
                    },
                    '#' => {}, // do nothing
                    _ => {
                        box_grid[current.y][current.x] = '.';
                        current.x = current.x-1;
                        box_grid[current.y][current.x] = '@';
                    }
                }
            }
            _ => panic!("invalid direction {:?}", direction)
        }

        // print(&box_grid);
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

fn build_input2(file: &str) -> Input {
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
                match chars[i] {
                    '@' => {
                        ch_vec.push('@');
                        ch_vec.push('.');
                    },
                    '#' => {
                        ch_vec.push('#');
                        ch_vec.push('#');
                    },
                    '.' => {
                        ch_vec.push('.');
                        ch_vec.push('.');
                    },
                    'O' => {
                        ch_vec.push('[');
                        ch_vec.push(']');
                    },
                    _ => panic!("Invalid char: {}", ch)
                }
            }
            box_grid.push(ch_vec);
        }
    });

    Input { box_grid, robot_path, start }
}
