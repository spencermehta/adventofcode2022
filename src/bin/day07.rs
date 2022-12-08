fn main() {
    let mut stack = vec![
        ("/", 0)
    ];

    let input = include_str!("./input.prod");
    let mut sum = 0;
    let threshold = 100000;
    let fs = 70000000;
    let need_unused = 30000000;
    let mut all = vec![
        ("/", 0)
    ];

    for line in input.lines().skip(1) {
        if line.starts_with("$ ls") {
            continue
        }

        if line == "$ cd .." {
            let (a, b) = stack.pop().expect("exists");
            if b <= threshold {
                sum += b;
            }
            all.push((a, b));
            stack.last_mut().expect("blah").1 += b;
            continue;
        }

        if line.starts_with("$ cd ") {
            stack.push((&line[5..], 0));
            continue;
        }

        let (a, _) = line.split_once(' ').expect("must work");

        if a == "dir" {
            continue;
        }

        stack.last_mut().expect("blah").1 += a.parse::<u32>().expect("blah");
    }
    while stack.len() > 1 {
        let (a, b) = stack.pop().expect("exists");
        stack.last_mut().expect("blah").1 += b;
        all.push((a, b));
    }
    let (a, b) = stack.pop().expect("exists");

    let must_be_under: u32 = fs - need_unused;
    
    let ayy = all.into_iter().filter(|(_,y)| {println!("{}, {}", b, y); b - y <= must_be_under}).map(|(_,y)| y).min().expect("blah");

    println!("{:?}, {}, {}", stack, sum,  ayy)
}
