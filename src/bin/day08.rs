fn main() {
    let input = include_str!("./input.prod");
    let matrix = input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).expect("a"))
    });

    let mut visibles = matrix.clone().map(|row| row.map(|_| 0).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();


    for (i, row) in matrix.clone().enumerate() {
        let l = row.clone().count() - 1;
        visibles[i][0] = 1;
        visibles[i][l] = 1;

        let mut max_left = 0;
        for (j, col) in row.clone().enumerate() { 
            if col > max_left {
                visibles[i][j] = 1;
                max_left = col;
            }
        }

        let mut max_right = 0;
        for (j, col) in row.clone().rev().enumerate() { 
            if col > max_right {
                visibles[i][l - j] = 1;
                max_right = col;
            }
        }
    }
    let mat = matrix.map(|row| row.collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    for j in 0..visibles[0].len() {
        let l = visibles.len() - 1;
        visibles[0][j] = 1;
        visibles[l][j] = 1;

        let mut max_top = 0;
        for i in 0..visibles.len() {
            let cur = mat[i][j];

            if cur > max_top {
                visibles[i][j] = 1;
                max_top = cur;
            }
        }
        let mut max_bot = 0;
        for i in (0..visibles.len()).rev() {
            let cur = mat[i][j];
            println!("{}", i);

            if cur > max_bot {
                visibles[i][j] = 1;
                max_bot = cur;
            }
        }
    }

    //for row in &visibles {
    //    println!("{:?}", row);
    //}

    let t = visibles.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>();
    println!("{}", t);
}
