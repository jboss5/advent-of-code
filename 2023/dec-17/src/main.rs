
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use priority_queue::DoublePriorityQueue;

#[derive(Eq,PartialEq,Debug,Hash,Copy,Clone)]
enum Direction {
    North,
    East,
    West,
    South,
    None,
}

#[derive(Eq,PartialEq,Debug,Hash,Copy,Clone)]
struct Node {
    h: i32,
    w: i32,
    direction: Direction,
    cost: i32,
}

#[derive(Eq,PartialEq,Debug,Hash,Copy,Clone)]
struct Visited {
    node: Node,
    cost: i32,
}

fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("Error opening file")).lines()
}

fn build_grid(filename: &String) -> Vec<Vec<u32>> {
    let mut grid = vec![];

    for l in get_lines(filename) {
        let line = l.unwrap();
        grid.push(line.chars().map(|ch| ch.to_digit(10).unwrap()).collect());        
    }

    grid
}

fn get_neighbors(current: &Node, grid: &Vec<Vec<u32>>) -> Vec<Node> {
    // println!("n current: {:?}", current);

    let mut out = vec![];
    let w = current.w as usize;
    let h = current.h as usize;
    let w_32 = w as i32;
    let h_32 = h as i32;

    if h >= 1 { 
        let c1 = Node { h: h_32-1, w: w_32, direction: Direction::North, cost: 0 };
        out.push(c1); 
        // println!("c1: {:?}",c1);
    }
    if h <= grid.len()-2 { 
        let c2 = Node { h: h_32+1, w: w_32, direction: Direction::South, cost: 0 };
        out.push(c2); 
        // println!("c2: {:?}",c2);
    }
    if w >= 1 { 
        let c3 = Node { h: h_32, w: w_32-1, direction: Direction::West, cost: 0 };
        out.push(c3); 
        // println!("c3: {:?}",c3);
    }
    if w <= grid[0].len()-2 { 
        let c4 = Node { h: h_32, w: w_32+1, direction: Direction::East, cost: 0 };
        out.push(c4); 
        // println!("c4: {:?}",c4);
    }

    // println!("neighbors: {:?}", out);

    out
}

fn calc_distance(current: &Node, end_coord: &Node) -> i32 {
    // i32::abs(current.w - end_coord.w) + i32::abs(current.h - end_coord.h)
    (end_coord.w - current.w) + (end_coord.h - current.h)
}

fn get_direction(n1: &Node, n2: &Node) -> Direction {
    if n1.w == n2.w+1 {
        Direction::East
    } else if n1.h == n2.h+1 {
        Direction::South
    } else if n1.h == n2.h-1 {
        Direction::North
    } else if n1.w == n2.w-1 {
        Direction::West
    } else {
        Direction::None
    }
}

fn max_continues(current: &Node, next: &Node, visited: &HashMap<Node,Node>) -> bool {
    let previous_i = visited.get(current);
    if previous_i.is_none() { return false; }

    let previous_ii = visited.get(previous_i.unwrap());
    if previous_ii.is_none() { return false; }

    let previous_iii = visited.get(previous_ii.unwrap());
    if previous_iii.is_none() { return false; }
      
    let d_i = get_direction(previous_iii.unwrap(), previous_ii.unwrap());
    let d_ii = get_direction(previous_ii.unwrap(), previous_i.unwrap());
    let d_iii = get_direction(previous_i.unwrap(), current);
    let d_iv = get_direction(current, next);

    d_i == d_ii && d_ii == d_iii && d_iii == d_iv
}

fn p1(grid: &mut Vec<Vec<u32>>) -> u32 {
    let start = Node { h: 0_i32, w: 0_i32, direction: Direction::None, cost: 0 };
    let end = Node { h: (grid.len()-1) as i32, w: (grid[0].len()-1) as i32, direction: Direction::None, cost: 0 };
    // let end = Node { h: 0_i32, w: 0_i32, direction: Direction::None, cost: 0 };
    // let start = Node { h: (grid.len()-1) as i32, w: (grid[0].len()-1) as i32, direction: Direction::None, cost: 0 };
    println!("start: {:?}, end: {:?}", start, end);

    let mut queue: DoublePriorityQueue<Node, i32> = DoublePriorityQueue::new();
    queue.push(start, 1);

    let mut visited: HashMap<Node, Node> = HashMap::new();
    let mut costs = HashMap::new();
    costs.insert(start, 0);

    // let mut test: HashMap<Node, Vec<Visited>> = HashMap::new();

    while !queue.is_empty() {
        let current = queue.pop_min().unwrap();
        // println!("current: {:?}", current.0);

        // if current.0.w == end.w && current.0.h == end.h {
        //     break;
        // }

        for mut n in get_neighbors(&current.0, grid) {
            let cost = costs.get(&current.0).unwrap() + (grid[n.h as usize][n.w as usize]);
            println!("===n: {:?} || adding value: {}", n, grid[n.h as usize][n.w as usize]);
            // let visited_n = Visited { node: n, cost: cost as i32 };
            // let visited_c = Visited { node: current.0, cost: *costs.get(&current.0).unwrap() as i32 };
            let prev = visited.get(&current.0);
            if max_continues(&current.0, &n, &visited) { continue; }
        
            // test.entry(n).or_insert(vec![visited_n]).push(visited_n);

            // if !test.contains_key(&n) {
            //     let vec = test.get(&n).unwrap();
            //     vec.push(visited_n);
            //     test.insert(n, vec);
            // } else {
            //     test.insert(n.to_owned(), vec![visited_n]);
            // }

            if (!costs.contains_key(&n) || cost < *costs.get(&n).unwrap()) || !(prev.is_some() && prev.unwrap().h == n.h && prev.unwrap().w == n.w) {
                let d = calc_distance(&n, &end);
                // let d = 0;
                let priority = (cost as i32) + d;
                // println!("==next: {:?} || cost: {cost} || current cost: {} || cost2: {} || priority: {}", n, costs.get(&current.0).unwrap(), d, priority);
                // n.cost = cost as i32;
                println!("==inserting node: {:?} with cost: {cost}", n);

                queue.push(n, priority);
                visited.insert(n, current.0);
                costs.insert(n, cost);
            }
        }
    }

    let mut final_end = Node { h: end.h, w: end.w, direction: Direction::None, cost: 0 };
    let mut lowest = u32::MAX;
    for node in &visited {
        if node.0.h == end.h && node.0.w == end.w && lowest > *costs.get(&node.0).unwrap() {
            final_end = *node.0;
            lowest = *costs.get(&node.0).unwrap();
            println!("FINAL: {:?} - cost {} - visited {:?}", node, costs.get(&node.0).unwrap(), visited.get(node.0).unwrap());
        }
    }

    println!();

    for t in &visited {
        println!("{:?}", t);
    }

    println!();

    // println!("test:");
    // for v in test {
    //     println!("{:?} -> {:?}", v.0, v.1);
    // }
    // println!();

    let mut s = final_end;
    let mut sum = 0;
    while s != start {
        // if s != final_end { 
            // println!("{:?}",s.node);
            sum += grid[s.h as usize][s.w as usize]; 
            grid[s.h as usize][s.w as usize] = 0;
        // }
        s = *visited.get(&s).unwrap();
    }

    // sum += grid[s.h as usize][s.w as usize]; 
    // grid[s.h as usize][s.w as usize] = 0;


    for row in grid {
        for v in row {
            print!("{:?}",v);
        } 
        println!();
    }
    println!();

    sum
}

fn main() {
    let filename = "input-sample.txt".to_owned();
    let mut grid = build_grid(&filename);
    println!("p1: {}", p1(&mut grid));
}


// 893
// 877
// 731