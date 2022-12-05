/*

[D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

 * */

use std::{string::ParseError, str::FromStr};

#[derive(Debug)]
struct Stack {
    items: Vec<char>
}

impl Stack {
    fn new(mut arr: Vec<char>) -> Self {
        arr.reverse();
        Self {
            items: arr
        }
    }

    fn pop(&mut self) -> Option<char> {
        self.items.pop()
    }

    fn pop_many(&mut self, num: u32) -> Vec<char> {
        let mut res = vec![];
        for i in 0..num {
            res.push(self.items.pop().expect("pop"))
        }
        res.reverse();
        res
    }

    fn push(&mut self, c: char) {
        self.items.push(c)
    }

    fn push_many(&mut self, chars: Vec<char>) {
        for c in chars {
            self.items.push(c)
        }
    }

    fn peek(&self) -> Option<&char> {
        self.items.last()
    }
}

#[derive(Debug)]
struct StackCollection {
    items: Vec<Stack>
}

impl StackCollection {
    fn new(matrix: Vec<Stack>) -> Self {
        Self {
            items: matrix
        }
    }

    fn apply_instr(&mut self, instr: Instr) {
        for n in 0..instr.num {
            let c = self.items[usize::try_from(instr.from).expect("u32 -> usize")].pop().expect("pop");
            self.items[usize::try_from(instr.to).expect("u32 -> usize")].push(c)
        }
    }

    fn apply_part_2_instr(&mut self, instr: Instr) {
       let cs = self.items[usize::try_from(instr.from).expect("u32 -> usize")].pop_many(instr.num);
       self.items[usize::try_from(instr.to).expect("u32 -> usize")].push_many(cs)
    }

    fn get_tops(&self) -> Vec<&char> {
        self.items.iter().map(|x| x.peek().expect("peek")).collect()
    }
}

impl FromStr for StackCollection {
    type Err = ParseError;

    /*
    
        [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    01234567890
     * */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matrix: Vec<Vec<char>> = vec![];

        for (i, line) in s.lines().enumerate() {
            let l = line.chars();
            for (j, c) in line.chars().skip(1).enumerate() {
                let pos = j % 4;
                if (pos != 0) || (c == ' ') || !c.is_uppercase() {
                    continue
                }
                let index = j / 4;
                while matrix.len() <= index {
                    matrix.push(vec![])
                }
                println!("{}, {}", index, matrix.len());
                matrix[index].push(c);
            }
        }
        Ok(StackCollection::new(matrix.into_iter().map(|x| Stack::new(x)).collect()))
    }

}

#[derive(Debug)]
struct Instr {
    from: u32,
    to: u32,
    num: u32
}

impl Instr {
    fn new(arr: Vec<&str>) -> Self {
        Self {
            num: arr[0].parse().expect("instr num"),
            from: arr[1].parse::<u32>().expect("instr from") - 1,
            to: arr[2].parse::<u32>().expect("instr to") - 1,
        }
    }
}

impl FromStr for Instr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instr::new(s.split(' ').filter(|x| x.chars().all(|x| x.is_numeric())).collect()))
    }
}

fn main() {
    let input = include_str!("./input.prod");
    let (a,b) = input.split_once("\n\n").expect("should be splittable");
    let mut stack_collection = StackCollection::from_str(a).expect("pls work");
    println!("{:?}\n", stack_collection);

    let instrs: Vec<Instr> = b.lines().map(|line| Instr::from_str(line).expect("pls work instr")).collect();
    println!("{:?}\n", instrs);

    for instr in instrs {
        println!("{:?}", instr);
        // stack_collection.apply_instr(instr);
        stack_collection.apply_part_2_instr(instr);
        println!("{:?}\n", stack_collection);
    }

    let tops: String = stack_collection.get_tops().into_iter().collect();
    println!("{}", tops);

}
