use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input-file.txt").expect("Error opening input file");
    let reader = BufReader::new(file);
    let mut total_p1 = 0_i32;
    let mut total_p2 = 0_i32;

    for line in reader.lines() {
        let ll = line.unwrap();
        let sp = ll.split(' ').collect::<Vec<&str>>();
        let d0 = sp[0].chars().next().unwrap();
        let d1 = sp[1].chars().next().unwrap();

        total_p1 += get_score(d0, d1);
        total_p2 += get_score(d0, find_hand(d0, d1).unwrap());
    }

    println!("p1 score: {}", total_p1);
    println!("p2 score: {}", total_p2);
}

fn get_score(opp: char, resp: char) -> i32 {
    get_hand_score(resp).unwrap() + get_game_score(opp, resp)
}

fn get_hand_score(hand: char) -> Option<i32> {
    match &hand {
        'X' => Some(1),
        'Y' => Some(2),
        'Z' => Some(3),
        _ => None
    }
}

fn get_game_score(opp: char, resp: char) -> i32 {
    let mut concat = String::new();
    concat.push(opp);
    concat.push(resp);

    match concat.as_str() {
        "AY" | "BZ" | "CX" => 6,
        "AX" | "BY" | "CZ" => 3,
        _ => 0
    }
}

fn find_hand(opp: char, resp: char) -> Option<char> {
    let mut concat = String::new();
    concat.push(opp);
    concat.push(resp);

    match concat.as_str() {
        "AY" | "BX" | "CZ" => Some('X'),
        "BY" | "CX" | "AZ" => Some('Y'),
        "CY" | "AX" | "BZ" => Some('Z'),
        _ => None
    }
}
