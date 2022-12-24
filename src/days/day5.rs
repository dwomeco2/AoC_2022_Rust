use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day5");

pub fn part1() {
    let (it, mut v) = process();

    for (count, from, to) in it {
        for _ in 0..count {
            let e = v[from - 1].pop().expect("Failed to Pop");
            v[to - 1].push(e);
        }
    }

    let result: String = v.map(|e| *e.last().unwrap() as char).iter().collect();
    println!("{result}");
}

pub fn part2() {
    let (it, mut v) = process();

    for (count, from, to) in it {
        let from_v = &mut v[from - 1];
        let s = from_v.drain((from_v.len() - count)..).collect_vec();
        v[to - 1].extend_from_slice(&s);
    }

    let result: String = v.map(|e| *e.last().unwrap() as char).iter().collect();
    println!("{result}");
}

fn process() -> (impl Iterator<Item = Move>, [Vec<u8>; 9]) {
    let mut it = INPUT.lines();
    let a = it.by_ref().take(8).collect_vec();

    let mut v: [Vec<u8>; 9] = Default::default();

    a.iter().rev().for_each(|l| {
        let b = l.as_bytes().chunks(4).map(|s| s[1]).enumerate();
        for (i, c) in b {
            if !c.is_ascii_whitespace() {
                v[i].push(c);
            }
        }
    });

    (it.skip(2).map(parse_move), v)
}

type Move = (usize, usize, usize);

fn parse_move(s: &str) -> Move {
    s.split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .collect_tuple::<Move>()
        .unwrap()
}
