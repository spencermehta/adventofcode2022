use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.prod");

    let mut registers: HashMap<char, i64> = HashMap::new();
    registers.insert('x', 1);

    let mut cycle = 0;

    let mut pixels: [char; 240] = ['.'; 240];

    for line in input.lines() {
        println!("{}", cycle);
        let x: i64 = *registers.get(&'x').expect("x");
        let pix = cycle % 40;
        if (pix == x-1) || (pix == x) || (pix == x+1) {
            let us = usize::try_from(cycle).expect("a");
            pixels[us] = '#';
        }

        if line.starts_with("noop") {
            cycle += 1;
            continue;
        }

        if line.starts_with("add") {
            cycle += 1;
            // do
            let x: i64 = *registers.get(&'x').expect("x");
            let pix = cycle % 40;
            if (pix == x-1) || (pix == x) || (pix == x+1) {
                let us = usize::try_from(cycle).expect("a");
                pixels[us] = '#';
            }
            // od
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
    println!("\n{}", "");

    for i in 0..240 {
        if i % 40 == 0 {
            println!();
        }
        let c = pixels[i];
        print!("{}", c);

    }
}
