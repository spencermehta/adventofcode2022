fn points(c: &str) -> isize {
    match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("invalid char"),
    }
}

/*
a > c > b > a
(x - y) % 3
*/

fn score(arr: &[isize]) -> isize {
    let res = (arr[1] - arr[0] + 1).rem_euclid(3) - 1;
    (res + 1) * 3 + arr[1] + 1
}

fn score_b(arr: &[isize]) -> isize {
    let res = (arr[0] + arr[1] - 1).rem_euclid(3) + 1;
    res + 3 * arr[1]
}

fn main() {
    let input = include_str!("input.txt");
    let rounds = input.lines().map(|line| line.split(' '));
    let r1 = rounds.clone();
    let scores: Vec<isize> = r1
        .map(|round| {
            let positions = round.map(points).collect::<Vec<isize>>();
            score(&positions)
        })
        .collect();

    println!("{:?}", scores.iter().sum::<isize>());

    let scores: Vec<isize> = rounds
        .map(|round| {
            let positions = round.map(points).collect::<Vec<isize>>();
            score_b(&positions)
        })
        .collect();

    println!("{:?}", scores.iter().sum::<isize>())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_score() {
        let arr = vec!["A", "Y"];
        let scores: Vec<isize> = arr.iter().map(|x| points(x)).collect();
        assert_eq!(8, score(&scores));

        let arr = vec!["B", "X"];
        let scores: Vec<isize> = arr.iter().map(|x| points(x)).collect();
        assert_eq!(1, score(&scores));

        let arr = vec!["C", "Z"];
        let scores: Vec<isize> = arr.iter().map(|x| points(x)).collect();
        assert_eq!(6, score(&scores));
    }
}
