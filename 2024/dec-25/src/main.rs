use aoc_utils::get_lines_str;

fn main() {
    println!("Hello, world!");
}

fn build_input(file: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut lock_vec = vec![];
    let mut key_vec = vec![];
    
    get_lines_str(file).for_each(|l| {
        let line = l.unwrap();
        
    });

    (lock_vec, key_vec)
}