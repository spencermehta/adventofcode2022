use std::{str::FromStr, string::ParseError, collections::VecDeque};
use num::integer::lcm;

#[derive(Debug)]
enum Op {
    Plus,
    Mult
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Plus,
            "*" => Self::Mult,
            _ => panic!("unexpected operator")
        })
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    New(u64),
}

impl FromStr for Operand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Self::Old,
            x => Self::New(x.parse::<u64>().expect("num pasre"))
        })
    }
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    queue: VecDeque<u64>,
    test: u64,
    op: Op,
    operand: Operand,
    t: u64,
    f: u64,
    num_inspected: u64,
    lcm: u64,
}

impl Monkey {
    fn inspect(&self, item: u64) -> u64 {
        let operand = match &self.operand {
            Operand::Old => item,
            Operand::New(x) => *x,
        };

        match &self.op {
            Op::Plus => {
                println!("adding {} to {}", item, operand);
                (item + operand) % self.lcm
            },
            Op::Mult => {
                println!("multiplying {} by {}", item, operand);
                (item * operand) % self.lcm
            }
        }
    }

    fn monkey_to_pass(&self, item: u64) -> u64 {
        if item % self.test == 0 {
            self.t
        } else {
            self.f
        }
    }

    fn take_turn(&mut self) -> (u64, u64) {
        let item = self.queue.pop_front().unwrap();
        let worry = self.inspect(item);
        self.num_inspected += 1;
        let new_monkey = self.monkey_to_pass(worry);
        println!("monkey {} inspected {} to {}, moving to {}. inspected {} | {:?}", self.id, item, worry, new_monkey, self.num_inspected, self.queue);
        (new_monkey, worry)
    }

    fn has_items(&self) -> bool {
        self.queue.len() > 0
    }
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id: u64 = lines.nth(0).expect("0th").split_once(' ').expect("split space").1.replace(':', "").parse().expect("prse");
        let starting_items: VecDeque<u64> = lines.nth(0).expect("1st").split_once(':').expect("split once").1.replace(' ', "").split(',').map(|x| x.parse::<u64>().expect("u64")).collect();

        let arithm = lines.nth(0).expect("2nd").split_once("= old ").expect("split").1.split_once(' ').expect("spl");
        let op: Op = arithm.0.parse().expect("op parse");
        let operand: Operand = arithm.1.parse().expect("operand parse");

        let divisible = lines.nth(0).expect("3rd").split(' ').nth(5).expect("int").parse::<u64>().expect("parseint");
        let true_monkey: u64 = lines.nth(0).expect("4th").split(' ').nth(9).expect("int").parse().unwrap();
        let false_monkey: u64 = lines.nth(0).expect("5th").split(' ').nth(9).expect("int").parse().unwrap();

        Ok(Self {
            id,
            queue: starting_items,
            test: divisible,
            op,
            operand,
            t: true_monkey,
            f: false_monkey,
            num_inspected: 0,
            lcm: 0,
        })
    }
}

fn mylcm(arr: Vec<u64>) -> u64 {
    let mut cur = arr[0];
    for el in arr.iter().skip(1) {
        cur = lcm(cur, *el);
    }
    cur
}

fn main() {
    let input = include_str!("./input.prod");
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|m| {
        m.parse::<Monkey>().expect("monkey parse")
    }).collect();

    for monkey in &monkeys {
        println!("{:?}", monkey);
    }

    let num_rounds = 10000;

    let mut divs: Vec<u64> = vec![];
    for monkey in &monkeys {
        divs.push(monkey.test);
    }

    let lcm = mylcm(divs);
    for monkey in &mut monkeys {
        monkey.lcm = lcm;
    }

    for round in 1..=num_rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].has_items() {
                let (new_monkey, item) = monkeys[i].take_turn();
                let index = usize::try_from(new_monkey).unwrap();
                monkeys[index].queue.push_back(item);
            }
        }
        println!("\nEnd of round {}:", round);
        for monkey in &monkeys {
            println!("{} {:?} {}", monkey.id, monkey.queue, monkey.num_inspected);
        }
    }

    monkeys.sort_by(|a, b| b.num_inspected.partial_cmp(&a.num_inspected).unwrap());
    let top_monkeys = monkeys.iter().take(2).map(|m| m.num_inspected);
    // for m in &top_monkeys {
    //     println!("{}", m);
    // }

    let monkey_business: u64 = top_monkeys.product();
    println!("{}", monkey_business);
}
