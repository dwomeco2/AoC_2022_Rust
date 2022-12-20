use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day6");

pub fn part1() {
    let sop_len = 4;
    let result = INPUT
        .as_bytes()
        .windows(sop_len)
        .position(|w| w.iter().all_unique());
    println!("{}", result.unwrap() + sop_len);
}

pub fn part2() {
    let sop_len = 14;
    let result = INPUT
        .as_bytes()
        .windows(sop_len)
        .position(|w| w.iter().all_unique());
    println!("{}", result.unwrap() + sop_len);
}

#[allow(dead_code)]
fn find_disjoint_window(s: &[u8], w: usize) -> Option<usize> {
    let mut last_known_position = [0; 256];
    let mut start_disjoint = 0;
    for i in 0..s.len() {
        start_disjoint = start_disjoint.max(last_known_position[s[i] as usize] + 1);
        last_known_position[s[i] as usize] = i;
        if i >= start_disjoint + w {
            return Some(i);
        }
    }
    None
}
