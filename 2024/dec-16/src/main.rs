use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use aoc_utils::{get_lines_str, man_dist, Coord};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd, Hash)]
enum Direction {
    NORTH,
    EAST,
    WEST,
    SOUTH,
    NONE
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
struct Node {
    coord: Coord,
    direction: Direction
}

// impl Hash for Node {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.coord.hash(state);
//     }
// }

struct PathFinder {
    grid: Vec<Vec<char>>,
    start: Node,
    end: Node,
    visited: HashSet<Coord>,
    cost_map: HashMap<(Coord,Direction), usize>,
    path_map: HashMap<(Coord,Direction), (Coord,Direction)>
}

impl PathFinder {
    pub fn new(grid: &Vec<Vec<char>>, start: Node, end: Node) -> PathFinder {
        PathFinder { grid: grid.clone(), start, end, visited: HashSet::<Coord>::new(), cost_map: HashMap::new(), path_map: HashMap::new() }
    }

    // pub fn get_path_map(self) -> HashMap<Node, Node> {
    //     self.path_map
    // }

    fn get_paths(&self, node: (Coord, Direction)) -> Vec<(Coord, Direction)> {
        let mut out = vec![];
        let y = node.0.y;
        let x = node.0.x;
        if node.1 != Direction::SOUTH && y > 0 && self.grid[y-1][x] != '#' { out.push((Coord{x, y: y-1}, Direction::NORTH)); }
        if node.1 != Direction::NORTH && y < self.grid.len()-1 && self.grid[y+1][x] != '#' { out.push((Coord{x, y: y+1}, Direction::SOUTH)); }
        if node.1 != Direction::WEST && x < self.grid[y].len()-1 && self.grid[y][x+1] != '#' { out.push((Coord{x: x+1, y }, Direction::EAST)); }
        if node.1 != Direction::EAST && x > 0 && self.grid[y][x-1] != '#' { out.push((Coord{x: x-1, y }, Direction::WEST)); }

        // if y > 0 && self.grid[y-1][x] != '#' { out.push((Coord{x, y: y-1}, Direction::NORTH)); }
        // if x > 0 && self.grid[y][x-1] != '#' { out.push((Coord{x: x-1, y }, Direction::WEST)); }
        // if y < self.grid.len()-1 && self.grid[y+1][x] != '#' { out.push((Coord{x, y: y+1}, Direction::SOUTH)); }
        // if x < self.grid[y].len()-1 && self.grid[y][x+1] != '#' { out.push((Coord{x: x+1, y }, Direction::EAST)); }

        out
    }

    pub fn run(&mut self, start: Node, direction: Direction) -> Option<Node> {
        // let mut queue = BinaryHeap::<(Coord,Direction)>::new();
        // let mut queue = PriorityQueue::<(Coord,Direction), usize>::new();
        let mut queue = BTreeSet::new();
        self.path_map.insert((self.start.coord, Direction::EAST),(self.start.coord, Direction::EAST));
        self.cost_map.insert((self.start.coord, self.start.direction), 0);
        // queue.push((self.start.coord, Direction::EAST), 0);
        queue.insert((0usize, (self.start.coord, self.start.direction)));

        while !queue.is_empty() {
            let node = queue.pop_first().unwrap();
            let pos = (node.1.0, node.1.1);

            if node.1.0 == self.end.coord {
                println!("found end {:?}", node.1);
                return Some(Node { coord: node.1.0, direction: node.1.1 });
            }

            if node.1.0.y == 7 && node.1.0.x == 9 {
                println!("here");
            }

            self.get_paths((node.1.0, node.1.1)).iter().for_each(|n| {

                // let old_cost = self.cost_map.get(&(n.0,n.1)).or(Some(&0)).unwrap();
                // // let cost =  (man_dist(&node.0, &n.0) as usize) + old_cost;
                // let cost =  (man_dist(&node.0.0, &n.0) as usize) + old_cost + calc_cost(&node, &n) + (man_dist(&node.0.0, &self.start.coord) as usize);
                // // let cost = calc_cost(&node.0, &n) + old_cost;
                // println!("curr: {:?} node {:?}, direction {:?}, old_cost {}, cost {}", node.0, n.0, n.1, old_cost, cost);
                // if (n.0.y == 9) && (n.0.x == 10) {
                //     println!("here {:?} {:?} cost {}", node, n, cost);
                // }


                let dist = man_dist(&self.end.coord, &n.0) as usize;
                let n_curr = calc_cost(&node.1, &n);
                let mut curr = *self.cost_map.get(&pos).unwrap_or(&0) + calc_cost(&node.1, &n);
                if n_curr < curr { curr = n_curr; }
                let prev = *self.cost_map.get(&n).unwrap_or(&usize::MAX);
                if curr < prev {
                    queue.insert((curr+dist, *n));
                    println!("aaaaa {:?} {:?} {}", (n.0, n.1), (node.1.0, node.1.1), curr);
                    self.path_map.insert((n.0, n.1), (node.1.0, node.1.1));

                    if node.1.0.y == 7 && node.1.0.x == 9 {
                        println!("here");
                    }


                    self.cost_map.insert((n.0,n.1), curr);
                }

                // if !self.cost_map.contains_key(&(n.0,n.1)) || cost < *old_cost {
                //     self.cost_map.insert((n.0,n.1), cost);
                //     // let priority = cost + (man_dist(&n.0, &self.end.coord) as usize) + calc_cost2(&node.0, &n);
                //     // let priority = calc_cost2(&node.0, &n);
                //     let priority = cost + (man_dist(&self.end.coord, &n.0) as usize);
                //     queue.push(*n, priority);
                //     if (n.0.y == 9) && (n.0.x == 9) {
                //         println!("here {:?} {:?} cost {} pr {}", node, n, cost, priority);
                //     }
                //     self.path_map.insert(Node { coord: n.0, direction: n.1 }, Node { coord: node.0.0, direction: node.0.1 });
                //     // self.visited.insert(n.0);
                // }
            });
        }

        println!("bad");
        None
    }
}

fn calc_cost2(n0: &(Coord, Direction), n1: &&(Coord, Direction)) -> usize {
    match n0.1 {
        Direction::NORTH => {
            match n1.1 {
                Direction::NORTH => 2000,
                Direction::EAST => 1000,
                Direction::WEST => 1000,
                _ => 1
            }
        },
        Direction::SOUTH => {
            match n1.1 {
                Direction::SOUTH => 2000,
                Direction::EAST => 1000,
                Direction::WEST => 1000,
                _ => 1,
            }
        },
        Direction::EAST => {
            match n1.1 {
                Direction::EAST => 2000,
                Direction::NORTH => 1000,
                Direction::SOUTH => 1000,
                _ => 1,
            }
        },
        Direction::WEST => {
            match n1.1 {
                Direction::WEST => 2000,
                Direction::NORTH => 1000,
                Direction::SOUTH => 1000,
                _ => 1,
            }
        }
        _ => panic!("invalid direction")
    }


    // match n0.1 != n1.1 {
    //     false => 1000,
    //     true => 1
    // }
}

fn calc_cost(n0: &(Coord, Direction), n1: &&(Coord, Direction)) -> usize {
    match n0.1 {
        Direction::NORTH => {
            match n1.1 {
                Direction::NORTH => 1,
                Direction::EAST => 1000,
                Direction::WEST => 1000,
                _ => 2000
            }
        },
        Direction::SOUTH => {
            match n1.1 {
                Direction::SOUTH => 1,
                Direction::EAST => 1000,
                Direction::WEST => 1000,
                _ => 2000,
            }
        },
        Direction::EAST => {
            match n1.1 {
                Direction::EAST => 1,
                Direction::NORTH => 1000,
                Direction::SOUTH => 1000,
                _ => 2000,
            }
        },
        Direction::WEST => {
            match n1.1 {
                Direction::WEST => 1,
                Direction::NORTH => 1000,
                Direction::SOUTH => 1000,
                _ => 2000,
            }
        }
        _ => panic!("invalid direction")
    }


    // match n0.1 != n1.1 {
    //     true => 1000,
    //     false => 1
    // }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }

        println!();
    }
}

fn print_path(grid: &Vec<Vec<char>>, start: Node, end: Node, visited: &HashMap<(Coord,Direction),(Coord,Direction)>, cost_map: &HashMap<(Coord,Direction), usize>) -> usize {
    let mut new_grid = grid.clone();
    let mut node = (end.coord, end.direction);
    let mut cost = 0_usize;
    loop {
        let next_node = visited.get(&node);
        println!("s: {:?} e: {:?}", node, next_node);
        new_grid[node.0.y][node.0.x] = 'x';
        cost += cost_map.get(&node).unwrap();
        // cost += 1;

        // if next_node.unwrap().direction != node.direction {
        //     println!("direction");
        //     cost += 1000; // already added 1
        // }

        if next_node.is_none() {
            println!("NOT FOUND {:?}", node);
            break;
        }

        if next_node.unwrap().0 == start.coord || next_node.unwrap().0 == end.coord { break; }
        node = *next_node.unwrap();
    }

    print_grid(&new_grid);
    cost
}

fn find_start_end(grid: &Vec<Vec<char>>) -> (Coord,Coord) {
    let mut start_end = (Coord { x: 0, y: 0}, Coord { x: 0, y: 0 });
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'E' {
                start_end.1 = Coord { y: y, x: x };
            } else if grid[y][x] == 'S' {
                start_end.0 = Coord { x: x, y: y };
            }
        }
    }

    start_end
}

fn main() {
    let grid = build_input("input-sample.txt");

    print_grid(&grid);
    let start = find_start_end(&grid);
    let start_node = Node { coord: start.0, direction: Direction::EAST };
    let end_node = Node { coord: start.1, direction: Direction::NONE };
    let mut path_finder = PathFinder::new(&grid, start_node, end_node);
    let end = path_finder.run(Node { coord: start.0, direction: Direction::EAST }, Direction::NORTH).unwrap();

    println!("{:?}", path_finder.path_map);
    let cost = print_path(&grid, start_node, end, &path_finder.path_map, &path_finder.cost_map);
    println!("Cost: {}", cost);
}


//133536
//132544
//182736
//222724
//645112
//106484 high
//226796

fn build_input(file: &str) -> Vec<Vec<char>> {
    get_lines_str(file).map(|l|
        l.unwrap()
            .chars()
            .collect()
    ).collect()
}