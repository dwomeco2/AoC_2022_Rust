const INPUT: &str = include_str!("../../input/day2");

pub fn part1() {
    let result = INPUT
        .lines()
        .map(|r| outcome_score(r) + shape_score(&r[2..]))
        .sum::<u32>();
    println!("{result}");
}

pub fn part2() {
    let result = INPUT
        .lines()
        .map(|r| {
            let me = &prophet(r);
            outcome_score(me) + shape_score(&me[2..])
        })
        .sum::<u32>();
    println!("{result}");
}

fn outcome_score(round: &str) -> u32 {
    match round {
        "A Y" | "B Z" | "C X" => 6,
        "A X" | "B Y" | "C Z" => 3,
        _ => 0,
    }
}

fn shape_score(my_pick: &str) -> u32 {
    match my_pick {
        "X" => 1,
        "Y" => 2,
        _ => 3,
    }
}

fn prophet(round: &str) -> String {
    let mut s = String::from(&round[..2]);
    match round {
        "B X" | "A Y" | "C Z" => s.push('X'),
        "C X" | "B Y" | "A Z" => s.push('Y'),
        _ => s.push('Z'),
    }
    s
}
