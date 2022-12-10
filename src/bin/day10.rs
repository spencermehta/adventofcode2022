use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.prod");

    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut signal_strengths: HashMap<i64, i64> = HashMap::new();

    let mut cycle = 1;

    for line in input.lines() {
        // println!("cycle {} instr {}", cycle, line);
        if (cycle + 20) % 40 == 0 {
            let x = registers.get(&'x').expect("x");
            let score = cycle * x;
            println!("cycle {} x {}", cycle, x);
            signal_strengths.insert(cycle, score);
        }

        if line.starts_with("noop") {
            cycle += 1;
            continue;
        }

        if line.starts_with("add") {
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                let x = registers.get(&'x').expect("x");
                let score = cycle * x;
                println!("cycle {} x {}", cycle, x);
                signal_strengths.insert(cycle, score);
            }
            cycle += 1;

            let reg = line.chars().nth(3).expect("aoc input");
            let num: i64 = line.split_once(' ').expect("aoc input").1.parse().expect("parse");
            let prev = match registers.get(&reg) {
                None => 1,
                Some(x) => *x,
            };

            registers.insert(reg, prev + num);
        }
    }

    println!("{}, {:?}", cycle, registers);
    println!("{:?}", signal_strengths);
    let sum_strengths: i64 = signal_strengths.values().sum();
    println!("\n{}", sum_strengths);
}
