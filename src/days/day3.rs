use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day3");

pub fn part1() {
    let result = INPUT
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(f, s)| f.chars().find(|&c| s.contains(c)).map(priority).unwrap())
        .sum::<u32>();
    println!("{result}")
}

pub fn part2() {
    let result = INPUT
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            a.chars()
                .find(|&x| b.contains(x) && c.contains(x))
                .map(priority)
                .unwrap()
        })
        .sum::<u32>();
    println!("{result}");
}

fn priority(c: char) -> u32 {
    match c.is_ascii_lowercase() {
        true => ((c as u8 - b'a') + 1) as u32,
        _ => ((c as u8 - b'A') + 27) as u32,
    }
}
