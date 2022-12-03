use itertools::izip;
use std::collections::HashSet;

fn shared_item((l, r): (&str, &str)) -> HashSet<char> {
    let (short, long) = if l.len() > r.len() { (r, l) } else { (l, r) };

    let set: HashSet<char> = short.chars().collect();

    long.chars()
        .filter(|x| set.contains(x))
        .collect::<HashSet<char>>()
}

fn score(set: &HashSet<char>) -> u32 {
    let set = set.iter().map(|x| {
        let y = *x as u32;
        if y >= 97 {
            y - 96
        } else {
            y - 38
        }
    });

    set.reduce(|acc, el| acc + el).unwrap()
}

fn main() {
    let input = include_str!("input.prod");

    // part one
    // let backpacks = input.lines().map(|line| line.split_at(line.len() / 2));
    //let total = &backpacks
    //    .map(|(l, r)| {
    //        let shared_item = shared_item((l, r));
    //        score(&shared_item)
    //    })
    //    .reduce(|acc, el| acc + el);

    // part two
    let l1 = input.lines();
    let l2 = l1.clone();
    let l3 = l1.clone();
    let z = izip!(l1, l2.skip(1), l3.skip(2));
    let total = z
        .into_iter()
        .enumerate()
        .filter(|(a, _)| a % 3 == 0)
        .map(|(_, b)| b)
        .map(|(x, y, z)| {
            let shared_xy: HashSet<char> = shared_item((x, y));
            let shared_yz: HashSet<char> = shared_item((y, z));
            let shared_zx: HashSet<char> = shared_item((z, x));

            let inter1: HashSet<char> = shared_xy.intersection(&shared_yz).map(|x| *x).collect();
            let inter: HashSet<char> = inter1.intersection(&shared_zx).map(|x| *x).collect();
            println!("{:?}", inter);

            let score = score(&inter);
            println!("{}", score);
            score
        })
        .reduce(|acc, el| acc + el);

    println!("{}", total.unwrap());
}
