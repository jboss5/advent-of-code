use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let file = File::open("input.txt").expect("Error opening input file");
    let reader = BufReader::new(file);
    let mut calories = vec![0];
    let mut index = 0;

    for line in reader.lines() {
        match line.unwrap().parse::<i32>() {
            Ok(num) => calories[index] += num,
            Err(_) => {
                index += 1;
                calories.push(0);
            }
        }
    }

    calories.sort_by(|a,b| b.cmp(a));
    let top3sum = calories[0] + calories[1] + calories[2];
    println!("highest calorie count: {}", calories[0]);
    println!("top 3 sum: {}", top3sum);
    println!("time elapsed: {:.2?}", now.elapsed());
}