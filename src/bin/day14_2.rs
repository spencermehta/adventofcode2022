#[derive(Copy, Clone, PartialEq)]
enum Space {
    Rock,
    Air,
    Sand,
    Source,
}

fn print_matrix(matrix: &Vec<Vec<Space>>) {
    for row in matrix {
        print!("n");
        for space in row {
            match space {
                Space::Rock => print!("#"),
                Space::Air => print!("."),
                Space::Sand => print!("o"),
                Space::Source => print!("+"),
            }
        }
        println!();
    }
}

fn plot_seg((x,y): (usize, usize), (a,b): (usize, usize), matrix: &mut Vec<Vec<Space>>, offset: usize) {
    if x == a {
        let (small, big) = if y < b { (y, b) } else { (b, y) };

        for i in small..=big {
            matrix[i][x-offset] = Space::Rock;
        }
    } else if y == b {
        let (small, big) = if x < a { (x, a) } else { (a, x) };

        for i in small..=big {
            matrix[y][i-offset] = Space::Rock;
        }
    } else {
        panic!("not straight line");
    }
}

fn drop_sand(matrix: &mut Vec<Vec<Space>>, offset: usize, source: (usize, usize)) {
    let new_pos = (source.0, source.1 + 1);

    if let Some(Space::Air) = matrix.get(new_pos.1).and_then(|row| row.get(new_pos.0- offset)) {
        drop_sand(matrix, offset, new_pos);
        return;
    }
    if let Some(Space::Air) = matrix.get(new_pos.1).and_then(|row| row.get(new_pos.0-1 - offset)) {
        drop_sand(matrix, offset, (new_pos.0 - 1, new_pos.1));
        return;
    }
    if let Some(Space::Air) = matrix.get(new_pos.1).and_then(|row| row.get(new_pos.0+1 - offset)) {
        drop_sand(matrix, offset, (new_pos.0 + 1, new_pos.1));
        return;
    }
    let _ = matrix[source.1+1][source.0-offset];
    matrix[source.1][source.0- offset] = Space::Sand;
}

fn main() {
    let input = include_str!("./input.prod");
    let rocks = input.lines().map(|line| {
        line.split(" -> ").map(|seg| seg.split_once(',').map(|(x,y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())).unwrap()).collect::<Vec<(usize, usize)>>()
    }).collect::<Vec<Vec<(usize, usize)>>>();

    for rock in &rocks {
        for seg in rock {
            println!("{:?}", seg);
        }
        println!();
    }

    let (_, max_y_temp) = rocks.iter()
        .map(|rock| rock.iter().max_by(|(_,y1), (_, y2)| y1.cmp(&y2)).unwrap())
        .max_by(|(_,y1), (_, y2)| y1.cmp(&y2)).unwrap();

    let (max_x_temp, _) = rocks.iter()
        .map(|rock| rock.iter().max_by(|(x1,_), (x2, _)| x1.cmp(&x2)).unwrap())
        .max_by(|(x1,_), (x2, _)| x1.cmp(&x2)).unwrap();

    let (min_x_temp, _) = rocks.iter()
        .map(|rock| rock.iter().max_by(|(x1,_), (x2, _)| x2.cmp(&x1)).unwrap())
        .max_by(|(x1,_), (x2, _)| x2.cmp(&x1)).unwrap();

    let max_y = max_y_temp + 2;
    let max_x = max_x_temp + 300;
    let min_x = min_x_temp - 300;
    let offset = min_x;

    println!("{:?}, {:?}, {:?}", max_y, max_x, min_x);

    let mut row: Vec<Space> = vec![];
    for _ in 0..=(max_x-min_x) {
        row.push(Space::Air)
    }

    let mut matrix: Vec<Vec<Space>> = vec![];
    for _ in 0..=max_y {
        matrix.push(row.clone());
    }
    matrix[0][500 - offset] = Space::Source;

    for rock in &rocks {
        for i in 0..rock.len()-1 {
            plot_seg(rock[i], rock[i+1], &mut matrix, offset);
        }
    }

    for i in 0..=(max_x-min_x) {
        matrix[max_y][i] = Space::Rock;
    }
    
    print_matrix(&matrix);

    for i in 0..1000000 {
        println!("i {}", i+1);
        if matrix[0][500-offset] == Space::Sand {
            break
        }
        drop_sand(&mut matrix, offset, (500, 0));
        //if i % 100 == 0 {
            //print_matrix(&matrix);
        //}
    }
}
