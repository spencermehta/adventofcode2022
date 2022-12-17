use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Self>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(first), Self::Int(second)) => {
                first.cmp(second)
            },
            (Self::List(first), Self::List(second)) => {
                for i in 0..first.len().min(second.len()) {
                    let comp = first[i].cmp(&second[i]);
                    if comp != Ordering::Equal {
                        return comp;
                    }
                }
                first.len().cmp(&second.len())
            }
            (Self::Int(first), second) => {
                Packet::List(vec![Self::Int(*first)]).cmp(second)
            }
            (first, Self::Int(second)) => {
                first.cmp(&Packet::List(vec![Self::Int(*second)]))
            }
        }
    }
}

fn parse_packet(s: Vec<&str>) -> (Packet, Vec<&str>) {
    let mut inp = s.clone();

    let mut v: Vec<Packet> = vec![];

    while inp.len() > 0 {
        match inp[0] {
            "[" => {
                let (ps, rem) = parse_packet(inp[1..].to_vec());
                v.push(ps);
                inp = rem;
            },
            "]" => {
                break
            }
            num => {
                v.push(Packet::Int(num.parse::<u32>().unwrap()));
                inp = inp[1..].to_vec();
            },
        }
    };

    let x: Vec<&str> = if inp.len() > 0 {
        inp[1..].to_vec()
    } else {
        vec![]
    };

    (Packet::List(v), x)
}

fn main() {
    let input = include_str!("./input.prod");

    // pt1
    // let mut index = 1;
    // let mut indices: Vec<u32> = vec![];

    // for (left, right, _) in input.lines().tuples() {
    //     let l = left.replace('[', ",[,").replace(']', ",],");
    //     let ls = l.split(',').filter(|x| *x != "").skip(1).collect::<Vec<&str>>();
    //     let r = right.replace('[', ",[,").replace(']', ",],");
    //     let rs = r.split(',').filter(|x| *x != "").skip(1).collect::<Vec<&str>>();

    //     let (l_pack, _) = parse_packet(ls);
    //     let (r_pack, _) = parse_packet(rs);
    //     let comp = l_pack.cmp(&r_pack);
    //     if comp == Ordering::Less {
    //         indices.push(index)
    //     }
    //     println!("l: {:?}\nr: {:?}\n{:?}\n", l_pack, r_pack, comp);
    //     index += 1;
    // }

    // println!("sum {}", indices.iter().sum::<u32>())
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    let lines = input.lines().filter(|line| *line != "");
    let mut packets = lines.map(|line| {
        let l = line.replace('[', ",[,").replace(']', ",],");
        let ls = l.split(',').filter(|x| *x != "").skip(1).collect::<Vec<&str>>();
        let (pack, _) = parse_packet(ls);
        pack
    }).collect::<Vec<Packet>>();
    packets.push(div1);
    packets.push(div2);

    packets.sort();

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    let i1 = packets.iter().position(|x| x == &div1).unwrap();
    let i2 = packets.iter().position(|x| x == &div2).unwrap();
    
    println!("{:?}", (i1+1) * (i2+1));
}

