
use aoc_utils::obj3d::Rectangle3d;

fn main() {
    let dims = aoc_utils::get_lines(&"input.txt".to_string()).map(|l| {
        let line = l.unwrap();
        let mut split = line.split('x');
        let mut dim: Rectangle3d = Default::default();
        dim.set_width(split.next().unwrap().parse::<usize>().unwrap());
        dim.set_height(split.next().unwrap().parse::<usize>().unwrap());
        dim.set_length(split.next().unwrap().parse::<usize>().unwrap());

        dim
    }).collect::<Vec<Rectangle3d>>();

    println!("p1 {}", p1(&dims));
    println!("p2 {}", p2(&dims));
}

fn p1(dims: &Vec<Rectangle3d>) -> u64 {
    dims.iter().map(|d| d.area() + d.area_smallest_sides()).sum::<u64>()
}

fn p2(dims: &Vec<Rectangle3d>) -> u64 {
    dims.iter().map(|d| d.smallest_perimeter() + d.volume()).sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_test() {
        let temp = vec![Rectangle3d::new(2, 3, 4)];
        assert_eq!(p1(&temp), 58);

        let temp2 = vec![Rectangle3d::new(1, 1, 10)];
        assert_eq!(p1(&temp2), 43);
    }

    #[test]
    fn p2_test() {
        let temp = vec![Rectangle3d::new(2, 3, 4)];
        assert_eq!(p2(&temp), 34);

        let temp2 = vec![Rectangle3d::new(1, 1, 10)];
        assert_eq!(p2(&temp2), 14);
    }
}