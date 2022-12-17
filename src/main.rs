mod days;

use std::io::Result;

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
        _ => println!("Wrong input: ##{}##", user_input.as_str()),
    }

    Ok(())
}
