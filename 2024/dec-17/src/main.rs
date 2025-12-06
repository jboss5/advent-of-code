use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use std::ops::BitXor;
use aoc_utils::get_lines_str;
use rayon::iter::IntoParallelIterator;

#[derive(Debug)]
struct Program {
    a: u128,
    b: u128,
    c: u128,
    i_counter: usize,
    output: String
}

impl Program {
    pub fn new(a: u128, b: u128, c: u128) -> Program {
        Program { a, b, c, i_counter: 0, output: String::new() }
    }

    fn get_operand_value(&self, op_value: &u128) -> u128{
        match op_value {
            0 | 1 | 2 | 3 => *op_value,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid op_value: {op_value}")
        }
    }

    fn adv_base(&mut self, operand: &u128) -> u128 {
        let numerator = self.a;
        let denominator = 2_u128.pow(*operand as u32);
        numerator / denominator
    }

    fn adv(&mut self, operand: &u128) {
        self.a = self.adv_base(operand);
    }

    fn bxl(&mut self, value: &u128) {
        self.b = self.b.bitxor(value);
    }

    fn bst(&mut self, operand: &u128) {
        self.b = operand % 8;
    }

    fn jnz(&mut self, operand: usize) -> bool {
        if self.a == 0 { true }
        else {
            self.i_counter = operand;
            false
        }
    }

    fn bxc(&mut self) {
        self.b = self.b.bitxor(self.c);
    }

    fn out(&self, operand: &u128) -> String {
        (operand % 8).to_string()
    }

    fn bdv(&mut self, operand: &u128) {
        self.b = self.adv_base(operand);
    }

    fn cdv(&mut self, operand: &u128) {
        self.c = self.adv_base(operand);
    }

    fn add_output(&mut self, text: String) {
        if !self.output.is_empty() { self.output.push_str(","); }
        self.output.push_str(text.as_str());
    }

    pub fn compute(&mut self, operations: Vec<u128>) -> &String {
        loop {
            if self.i_counter > operations.len()-1 { break; }
            let mut should_increment = true;
            let operation = &operations[self.i_counter];
            let operand = &operations[self.i_counter + 1];
            let combo_operand = if *operand == 7 { 0 } else { self.get_operand_value(&operand) };

            match operation {
                0 => self.adv(&combo_operand),
                1 => self.bxl(&operand),
                2 => self.bst(&combo_operand),
                3 => should_increment = self.jnz(*operand as usize),
                4 => self.bxc(),
                5 => self.add_output(self.out(&combo_operand)),
                6 => self.bdv(&combo_operand),
                7 => self.cdv(&combo_operand),
                _ => panic!("invalid operation: {operation}")
            }

            if should_increment { self.i_counter += 2; }
        }

        &self.output
    }

    pub fn compute_find_start(&mut self, operations: &Vec<u128>, str_to_find: &str) -> &String {
        loop {
            if self.i_counter > operations.len()-1 { break; }
            let mut should_increment = true;
            let operation = &operations[self.i_counter];
            let operand = &operations[self.i_counter + 1];
            let combo_operand = if *operand == 7 { 0 } else { self.get_operand_value(&operand) };

            match operation {
                0 => self.adv(&combo_operand),
                1 => self.bxl(&operand),
                2 => self.bst(&combo_operand),
                3 => should_increment = self.jnz(*operand as usize),
                4 => self.bxc(),
                5 => {
                    let str = self.out(&combo_operand);
                    self.add_output(str);
                    // if !str_to_find.ends_with(self.output.as_str()) {
                        // println!("{} != {}", self.output.as_str(), str_to_find);
                        // break;
                    // }
                },
                6 => self.bdv(&combo_operand),
                7 => self.cdv(&combo_operand),
                _ => panic!("invalid operation: {operation}")
            }

            if should_increment { self.i_counter += 2; }
        }

        &self.output
    }
}

fn main() {
    let mut input = build_input(&"input.txt");
    println!("{:?}", input);

    let mut program = Program::new(input.0, input.1, input.2);
    let output = program.compute(input.3.clone());
    println!("part1 {}", output);

    let mut p2_ans = String::new();
    input.3.clone().iter().for_each(|v| {
        if !p2_ans.is_empty() { p2_ans.push_str(","); }
        p2_ans.push_str(v.to_string().as_str());
    });

    let mut output_2 = "";
    // let mut i = 105843710000000;

    let mut i = 326891802524996_u128;
    // while i < u128::MAX
        // if i % 8 != 0 { continue; }
        let mut program = Program::new(i as u128, input.1, input.2);
        let output_2 = program.compute_find_start(&input.3, &p2_ans);
        // if output_2.len() >= 19 { println!("val: {i} output2: {output_2}"); }
        // if output_2.ends_with("1,5,0,3,4,3,5,5,3,0") {
        // if output_2 == &p2_ans {
        // if output_2.starts_with("0") {
            println!("hit part2 {}", i);
            // return;
        // }

        i += 1;
    // }

    // [2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 3, 5, 5, 3, 0]

    // 19484269+12909892
    // 11224715561176504

    // 1503435530 -
    // 503435530 - 103279138
    // 03435530 - 12909892
    // 3435530 - 1613736
    // 435530 - 201717
    // 35530 - 25214
    // 5530 - 3145
    // 530 - 393
    // 30 - 49
    // 0  - 48

    //16256463507956129471
    //17762281412461944511
    //17869229989009379007
    //18248224448738578111
    //18252950593170132671
    //21022421022421022400

    //210224

    // (0..usize::MAX).into_par_iter().for_each(|i| {
    // // let i = 18250622790688919231_u128;
    // let i = (19484269_u128*12909892_u128) as u128;
    //     let mut program = Program::new(i as u128, input.1, input.2);
    //     let output_2 = program.compute_find_start(&input.3, &p2_ans);
    //     if output_2.len() >= 19 { println!("val: {i} output2: {output_2}"); }
    //     // println!("val: {i} output2: {output_2}");
    //     if output_2 == &p2_ans {
    //         println!("hit part2 {}", i);
    //         return;
    //     }

        // i += 1;
    // });

    // loop {
    //     program = Program::new(i, input.1, input.2);
    //     output_2 = program.compute_find_start(&input.3, &p2_ans);
    //     if output_2 == p2_ans {
    //         println!("hit part2 {}", i);
    //         break;
    //     }
    //
    //     i += 1;
    // }

    // (822083584..6576668672_u128).into_par_iter().for_each(|i| {
    //     // if i % 8 != 0 { continue; }
    //     let mut program = Program::new(i as u128, input.1, input.2);
    //     let output_2 = program.compute_find_start(&input.3, &p2_ans[12..]);
    //     // if output_2.len() >= 19 {
    //     //     println!("val: {i} output2: {output_2}");
    //     // }
    //     if output_2.ends_with("1,5,0,3,4,3,5,5,3,0") {
    //         // if output_2 == &p2_ans {
    //         // if output_2.starts_with("2,4,1,3,7,5,1,5") {
    //         println!("hit part2 {}", i);
    //         return;
    //     }
    // });

    println!("part2 {}", output_2);
}

fn build_input(file: &str) -> (u128, u128, u128, Vec<u128>) {
    let mut out = (0,0,0,vec![]);
    let mut lines = get_lines_str(file);
    out.0 = lines.next().unwrap().unwrap().split(": ").last().unwrap().parse::<u128>().unwrap();
    out.1 = lines.next().unwrap().unwrap().split(": ").last().unwrap().parse::<u128>().unwrap();
    out.2 = lines.next().unwrap().unwrap().split(": ").last().unwrap().parse::<u128>().unwrap();
    lines.next();
    out.3.extend(lines.next().unwrap().unwrap().split(": ").last().unwrap().split(",").map(|s| s.parse::<u128>().unwrap()));

    out
}