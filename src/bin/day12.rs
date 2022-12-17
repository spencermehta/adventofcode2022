use std::{collections::{VecDeque, HashMap, HashSet}, thread::sleep, time::Duration};

#[derive(Debug)]
struct Position {
    coords: (usize, usize),
    character: char
}

impl Position {
    fn x(&self) -> usize {
        self.coords.0
    }

    fn y(&self) -> usize {
        self.coords.1
    }

    fn height(&self) -> usize {
        match self.character {
            'S' => 'a' as usize,
            'E' => 'z' as usize,
            c => c as usize,
        }
    }
}

struct Board {
    matrix: Vec<Vec<Position>>,
}

impl Board {
    fn legal_neighbours(&self, pos: &Position) -> Vec<&Position> {
        let board_size = self.board_size();

        let x = pos.x();
        let y = pos.y();

        let mut m: Vec<&Position> = vec![];

        if x > 0 {
            // println!("x > 0: {}", x);
            let p = &self.matrix[y][x-1];
            if Board::add_position(pos, p) {
                //println!("adding {:?}", p);
                m.push(p);
            }
        }
    
        if y > 0 {
            // println!("y > 0: {}", y);
            let p = &self.matrix[y-1][x];
            if Board::add_position(pos, p) {
                // println!("adding {:?}", p);
                m.push(p);
            }
        }
    
        if x < board_size.0 - 1 {
            // println!("x < s: {}", x);
            let p = &self.matrix[y][x+1];
            if Board::add_position(pos, p) {
                // println!("adding {:?}", p);
                m.push(p);
            }
        }
    
        if y < board_size.1 - 1 {
            // println!("y < s: {}", y);
            let p = &self.matrix[y+1][x];
            if Board::add_position(pos, p) {
                // println!("adding {:?}", p);
                m.push(p);
            }
        }
    
        m
    }

    fn get_start(&self) -> &Position {
        let y: usize = self.matrix.iter().position(|y| y.iter().any(|p| p.character == 'S')).unwrap();
        let x: usize = self.matrix[y].iter().position(|x| x.character == 'S').unwrap();

        &self.matrix[y][x]
    }

    fn get_end(&self) -> &Position {
        let y: usize = self.matrix.iter().position(|y| y.iter().any(|p| p.character == 'E')).unwrap();
        let x: usize = self.matrix[y].iter().position(|x| x.character == 'E').unwrap();

        &self.matrix[y][x]
    }

    fn board_size(&self) -> (usize, usize) {
        (self.matrix[0].len(), self.matrix.len())
    }
    
    fn add_position(p1: &Position, p2: &Position) -> bool {
        // pt1
        // if p1.height() > p2.height() {
        //     return true;
        // }
        // p2.height() as u32 - p1.height() as u32 <= 1
        // pt2
        if p1.height() < p2.height() {
            return true;
        }
        p1.height() as u32 - p2.height() as u32 <= 1
    }
}

fn main() {
    let input = include_str!("./input.prod");
    let board = Board {
        matrix: input.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| Position {
                coords: (x, y),
                character: c,
            }).collect::<Vec<Position>>()
        }).collect::<Vec<Vec<Position>>>(),
    };
    
    // println!("{:?}", board.matrix);

    let start = board.get_start();
    let end = board.get_end();
    println!("start {:?} end {:?}", start, end);

    let mut queue: VecDeque<(&Position, usize)> = VecDeque::new();
    // pt1
    // queue.push_front((start, 0));
    queue.push_front((end, 0));

    let mut visited: Vec<Vec<bool>> = board.matrix.iter().map(|line| line.iter().map(|_| false).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
    visited[start.coords.1][start.coords.0] = true;

    while let Some((cur_pos, depth)) = queue.pop_front() {
        // println!("\n\n\n\n\n\n\n\n\n\n\n");
        // sleep(Duration::from_millis(10));
        for row in &visited {
            for &cell in row {
                print!("{}", if cell { '#' } else { '.' });
            }
            println!();
        }

        if cur_pos.character == 'a' {
            println!("solution: {}", depth);
            break;
        }

        let moves = board.legal_neighbours(cur_pos);
        for mv in moves {
            if !visited[mv.coords.1][mv.coords.0] {
                visited[mv.coords.1][mv.coords.0] = true;
                queue.push_back((mv, depth+1));
            }
        }
    }

    println!("{:?}", visited[end.coords.1][end.coords.0]);
    for row in visited {
        for cell in row {
            print!("{}", if cell { '#' } else { '.' });
        }
        println!();
    }

}

