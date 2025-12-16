use std::collections::{HashMap, HashSet};
use std::collections::BTreeMap;
use std::sync::Arc;
use aoc_utils::Coord3D;
use ordered_float::{Float, OrderedFloat};

fn main() {
    let junction_boxes = aoc_utils::get_lines(&"input.txt".to_string()).map(|l| {
        let line = l.unwrap();
        let mut tokens = line.split(',');
        Coord3D { 
            x: tokens.next().unwrap().parse::<i64>().unwrap(),
            y: tokens.next().unwrap().parse::<i64>().unwrap(),
            z: tokens.next().unwrap().parse::<i64>().unwrap()
        }
    }).collect::<Vec<Coord3D>>();

    println!("p1 {}", p1(&junction_boxes, 1000, false).0);
    println!("p2 {}", p1(&junction_boxes, 15000, true).1);
}

fn p1(junction_boxes: &Vec<Coord3D>, runs: usize, p2: bool) -> (u64,u64) {
    // I would prefer to do a map<coord, &set> then use the set ptr but rust makes things difficult
    let mut index = HashMap::new();
    let mut idx = 0_usize;
    let mut data: Vec<HashSet<Coord3D>> = junction_boxes.iter().map(|j| {
        let mut h = HashSet::new();
        h.insert(*j);
        index.insert(*j, idx);
        idx += 1;
        h
    }).collect::<Vec<HashSet<Coord3D>>>();

    let mut map = BTreeMap::new();
    for i in 0..junction_boxes.len() {
        for k in 1..junction_boxes.len() {
            let c1 = junction_boxes[i];
            let c2 = junction_boxes[k];
            if c1 == c2 { continue; }
            let dist = OrderedFloat::from(aoc_utils::dist_3d(&c1, &c2));
            map.insert(dist, (c1,c2));
        }
    }

    let mut last_coords = (Default::default(), Default::default());
    let mut map_iter = map.iter().clone();
    for i in 0..runs {
        let it = if p2 {
            let mut next = map_iter.next();
            if next.is_none() {
                map_iter = map.iter().clone();
                next = map_iter.next();
            }

            next
        } else {
            map_iter.next()
        };

        let e = it.unwrap();

        let c1 = e.1.0;
        let c2 = e.1.1;

        if p2 {
            let mut max_data_len = 0_usize;
            for d in data.iter() {
                if d.len() > max_data_len {
                    max_data_len = d.len();
                }
            }

            if max_data_len == junction_boxes.len() {
                break;
            }
        }

        let c1_idx = *index.get(&c1).unwrap();
        let c2_idx = *index.get(&c2).unwrap();

        if c1_idx == c2_idx { continue; }

        for v in data[c2_idx].clone() {
            data[c1_idx].insert(v);
            index.insert(v, c1_idx);
        }

        data[c2_idx].clear();
        last_coords.0 = c1.clone();
        last_coords.1 = c2.clone();
    }

    data.sort_by(|a,b| b.len().cmp(&a.len()));
    // println!("test {:?}", data);

    println!("last coords {:?}", last_coords);
    ((data[0].len() * data[1].len() * data[2].len()) as u64, (last_coords.0.x * last_coords.1.x) as u64)
}