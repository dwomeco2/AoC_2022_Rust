mod days;

use std::io::Result;

extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() -> Result<()> {
    let mut user_input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut user_input)?;

    match user_input.trim() {
        "1_1" => days::day1::part1(),
        "1_2" => days::day1::part2(),
        "2_1" => days::day2::part1(),
        "2_2" => days::day2::part2(),
        "3_1" => days::day3::part1(),
        "3_2" => days::day3::part2(),
        "4_1" => days::day4::part1(),
        "4_2" => days::day4::part2(),
        "5_1" => days::day5::part1(),
        "5_2" => days::day5::part2(),
        "6_1" => days::day6::part1(),
        "6_2" => days::day6::part2(),
        "7_1" => days::day7::part1(),
        "7_2" => days::day7::part2(),
        "8_1" => days::day8::part1(),
        "8_2" => days::day8::part2(),
        "9_1" => days::day9::part1(),
        "9_2" => days::day9::part2(),
        "10_1" => days::day10::part1(),
        "10_2" => days::day10::part2(),
        "11_1" => days::day11::part1(),
        "11_2" => days::day11::part2(),
        "12_1" => days::day12::part1(),
        "12_2" => days::day12::part2(),
        "13_1" => days::day13::part1(),
        "13_2" => days::day13::part2(),
        "14_1" => days::day14::part1(),
        "14_2" => days::day14::part2(),
        _ => println!("Wrong input: ##{}##", user_input.as_str()),
    }

    Ok(())
}
