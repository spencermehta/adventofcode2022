use std::collections::HashSet;

use itertools::Itertools;

fn windows(inp: Vec<char>, size: usize) {
    for i in 0..inp.len()-size {
        let slice = &inp[i..i+size];
        let set: HashSet<&char> = HashSet::from_iter(slice);
        if set.len() == size {
            println!("{:?}\n{}", set, i + 14);
            break;
        }
    }
}

fn main() {
    let input = include_str!("./input.test").trim();

    // part 1
    for (i, (a, b, c, d)) in input.chars().tuple_windows::<(_, _, _, _)>().enumerate() {
        if ((a != b) && (a != c) && (a != d) && (b != c) && (b != d) && (c != d)) {
            println!("{}, {}, {}, {}", a, b, c, d);
            println!("{}", i + 4);
            break;
        }
    }

    // part 2
    let w = windows(input.chars().collect(), 14);
}
