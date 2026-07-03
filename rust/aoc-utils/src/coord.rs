
#[derive(Debug,Default,Copy,Clone,Hash,Eq,PartialEq)]
pub struct Coord {
    pub x: i64,
    pub y: i64
}

#[derive(Debug,Default,Copy,Clone,Hash,Eq,PartialEq)]
pub struct Coord3D {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

pub fn print_grid<T>(grid: &Vec<Vec<T>>) 
    where
        T: std::fmt::Display,
{
    for c in 0..grid.len() {
        for c2 in 0..grid[c].len() {
            print!("{}", grid[c][c2]);
        }
        println!();
    }
    println!();
}

pub fn man_dist(coord1: &Coord, coord2: &Coord) -> i64 {
    i64::abs((coord1.x - coord2.x) + (coord1.y - coord2.y))
}

pub fn man_dist_3d(coord1: &Coord3D, coord2: &Coord3D) -> i64 {
    i64::abs((coord1.x - coord2.x) + (coord1.y - coord2.y) + (coord1.z - coord2.z))
}

pub fn dist_3d(coord1: &Coord3D, coord2: &Coord3D) -> f32 {
    f32::sqrt((coord2.x - coord1.x).pow(2) as f32 
        + (coord2.y - coord1.y).pow(2) as f32
        + (coord2.z - coord1.z).pow(2) as f32
    )
}