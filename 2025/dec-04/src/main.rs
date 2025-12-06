use aoc_utils::do_while;


fn main() {
    let mut grid;
    grid = aoc_utils::get_lines(&"input.txt".to_string()).map(|l| {
        let mut line = l.unwrap();
        line.push(',');
        line.insert(0, ',');
        line.chars().collect()
    }).collect::<Vec<Vec<char>>>();

    grid.insert(0, vec![','; grid[0].len()]);
    grid.push(vec![','; grid[0].len()]);

    println!("p1 {}", p1(&grid));
    println!("p2 {}", p2(&mut grid));
}

fn is_accessible(paper_grid: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    vec![(y-1,x-1),(y-1,x),(y-1,x+1),(y,x-1),(y,x+1),(y+1,x-1),(y+1,x),(y+1,x+1)]
        .iter()
        .map(|(y,x)| if paper_grid[*y][*x] == '@' { 1 } else { 0 })
        .collect::<Vec<u64>>()
        .iter()
        .sum::<u64>() < 4
}

fn p1(paper_grid: &Vec<Vec<char>>) -> u64 {
    (1..paper_grid.len()-1).into_iter().map(|y| 
        (1..paper_grid[y].len()-1)
            .into_iter()
            .filter(|x| paper_grid[y][*x] == '@')
            .filter(|x| is_accessible(paper_grid, y, *x))
            .map(|_x| 1)
            .sum::<u64>()
        )
    .sum::<u64>()
}

fn p2(paper_grid: &mut Vec<Vec<char>>) -> u64 {
    let mut total = 0_u64;

    let mut no_rolls_moved;
    do_while!(!no_rolls_moved, {
        no_rolls_moved = true;
        for y in 1..paper_grid.len()-1 {
            for x in 1..paper_grid[y].len()-1 {
                if paper_grid[y][x] == '@' && is_accessible(paper_grid, y, x) {
                    paper_grid[y][x] = 'x';
                    total += 1;
                    no_rolls_moved = false;
                }
            }
        }
    });

    total
}