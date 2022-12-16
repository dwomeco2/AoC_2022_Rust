mod days;

use std::io::Result;

fn main() -> Result<()> {
    let mut user_input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut user_input)?;

    match user_input.trim() {
        "1_1" => days::day1::part1(),
        "1_2" => days::day1::part2(),
        _ => println!("Wrong input: ##{}##", user_input.as_str()),
    }

    Ok(())
}
