fn walk(mat: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let cur = mat[y][x];

    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bot = 0;

    for j in (0..x).rev() {
        left += 1;
        if mat[y][j] >= cur {
            break;
        } 
    }
    for j in x+1..mat.len() {
        right += 1;
        if mat[y][j] >= cur {
            break;
        } 
    }

    for i in (0..y).rev() {
        top += 1;
        if mat[i][x] >= cur {
            break;
        }
    }
    for i in y+1..mat.len() {
        bot += 1;
        if mat[i][x] >= cur {
            break;
        }
    }

    println!("({}, {}) {} - {} {} {} {}", y, x, cur, left, right, top, bot);
    left * right * top * bot
}

fn main() {
    let input = include_str!("./input.prod");
    let matrix = input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).expect("a"))
    });
    let mat = matrix.clone().map(|line| line.collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let mut scores = matrix.clone().map(|row| row.map(|_| 0).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    for y in 0..scores.len() {
        for x in 0..scores[0].len() {
            scores[y][x] = walk(&mat, x, y);
            println!("{}", scores[y][x]);
        }
    }

    //for row in &visibles {
    //    println!("{:?}", row);
    //}

    let t = scores.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    println!("{}", t);
}
