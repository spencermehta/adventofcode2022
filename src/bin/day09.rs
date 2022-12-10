use std::collections::HashSet;

fn move_tail(tail: (i64, i64), head: (i64, i64)) -> (i64, i64) {
    let mut new_tail = tail;

    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    if (x_diff.abs() + y_diff.abs()) >= 3 {
        if head.1 > tail.1 {
            new_tail.1 = new_tail.1 + 1;
        } else {
            new_tail.1 = new_tail.1 - 1;
        }
        if head.0 > tail.0 {
            new_tail.0 = new_tail.0 + 1;
        } else {
            new_tail.0 = new_tail.0 - 1;
        }
        return new_tail;
    }

    if x_diff.abs() > 1 {
        if head.0 > tail.0 {
            new_tail.0 = new_tail.0 + 1;
        } else {
            new_tail.0 = new_tail.0 - 1;
        }
    }

    if y_diff.abs() > 1 {
        if head.1 > tail.1 {
            new_tail.1 = new_tail.1 + 1;
        } else {
            new_tail.1 = new_tail.1 - 1;
        }
    }

    new_tail
}

fn main() {
    let input = include_str!("./input.prod");
    let instrs = input.lines().map(|line| {
        let (dir, num) = line.split_once(' ').expect("aoc inp to be corr");

        let n = num.parse::<i64>().expect("unwrap");

        match dir {
            "U" => (0, n),
            "D" => (0, -n),
            "R" => (n, 0),
            "L" => (-n, 0),
            _ => panic!("unexpected input")
        }
    });

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    visited.insert(tail_pos);

    for (x, y) in instrs {
        let x_neg = x < 0;
        for i in 0..x.abs() {
            let old_head_pos = head_pos;
            let old_tail_pos = tail_pos;

            let new_x = if x_neg {
                head_pos.0 - 1
            } else {
                head_pos.0 + 1
            };

            head_pos = (new_x, head_pos.1);

            tail_pos = move_tail(tail_pos, head_pos);
            visited.insert(tail_pos);

            println!("{:?} -> {:?}\t\t{:?} -> {:?} | {}", old_head_pos, head_pos, old_tail_pos, tail_pos, visited.len());
        }

        let y_neg = y < 0;
        for i in 0..y.abs() {
            let old_head_pos = head_pos;
            let old_tail_pos = tail_pos;

            let new_y = if y_neg {
                head_pos.1 - 1
            } else { 
                head_pos.1 + 1
            };

            head_pos = (head_pos.0, new_y);

            tail_pos = move_tail(tail_pos, head_pos);
            visited.insert(tail_pos);

            println!("{:?} -> {:?}\t\t{:?} -> {:?} | {}", old_head_pos, head_pos, old_tail_pos, tail_pos, visited.len());
        }

    }

    println!();

    for visit in &visited {
        println!("{:?}", visit);
    }
    println!("\n{}", visited.len());
}
