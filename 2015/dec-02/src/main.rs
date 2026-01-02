
#[derive(Default)]
struct Dimension {
    w: usize,
    h: usize,
    l: usize
}

fn main() {
    let dims = aoc_utils::get_lines(&"input.txt".to_string()).map(|l| {
        let line = l.unwrap();
        let mut split = line.split('x');
        let mut dim: Dimension = Default::default();
        dim.w = split.next().unwrap().parse::<usize>().unwrap();
        dim.h = split.next().unwrap().parse::<usize>().unwrap();
        dim.l = split.next().unwrap().parse::<usize>().unwrap();

        dim
    }).collect::<Vec<Dimension>>();

    println!("p1 {}", p1(dims));
}

fn p1(dims: Vec<Dimension>) -> u64 {
    
    dims.iter().map(|dim| {
        let mut vals = vec![];
        vals.push(dim.w * dim.h);
        vals.push(dim.h * dim.l);
        vals.push(dim.w * dim.l);
        vals.sort();

        (vals.iter().map(|v| 2*v).sum::<usize>() + vals[0]) as u64
    }).sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_test() {
        let temp = vec![Dimension { w: 2, h: 3, l: 4 }];
        assert_eq!(p1(temp), 58);

        let temp2 = vec![Dimension { w: 1, h: 1, l: 10 }];
        assert_eq!(p1(temp2), 43);
    }
}