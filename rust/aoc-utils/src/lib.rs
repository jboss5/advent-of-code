use std::{fs::File, io::{BufRead, BufReader, Lines}};
use std::str::FromStr;

pub mod coord;
pub mod obj3d;

pub fn str_to_vec<T: FromStr>(line: String, delim: &str) -> Vec<T> {
    Vec::from_iter(line.split(delim)
        .map(|s| T::from_str(s).ok().unwrap())
    )
}

pub fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("error opening file")).lines()
}

pub fn get_lines_str(filename: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("error opening file")).lines()
}

#[macro_export]
macro_rules! do_while {
    ($condition:expr, $code:block) => {
        loop {
            $code
            if !$condition { break; }
        }
    };
}