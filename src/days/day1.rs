use std::collections::BTreeSet;

const INPUT: &str = include_str!("../../input/day1");

pub fn part1() {
    let result = INPUT.split("\n\n")
        .map(|ls| ls.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap_or(0);
    println!("{result}");
}

pub fn part2() {
    let sg = INPUT.split("\n\n")
        .map(|ls| ls.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<BTreeSet<_>>();
    let result = sg.iter().rev().take(3).sum::<u32>();
    println!("{result}");
}
