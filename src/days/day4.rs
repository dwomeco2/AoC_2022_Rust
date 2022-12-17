use std::str::FromStr;

const INPUT: &str = include_str!("../../input/day4");

pub fn part1() {
    let result = extract()
        .filter(|s| is_fullcover(&s.first, &s.second))
        .count();
    println!("{result}");
}

pub fn part2() {
    let result = extract()
        .filter(|s| is_overlap(&s.first, &s.second))
        .count();
    println!("{result}");
}

struct Range {
    from: i32,
    to: i32,
}

struct Section {
    first: Range,
    second: Range,
}

#[derive(Debug)]
enum Error {
    ParseRangeError,
    ParseSectionError,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').ok_or(Error::ParseRangeError)?;
        Ok(Range {
            from: from.parse::<i32>().map_err(|_| Error::ParseRangeError)?,
            to: to.parse::<i32>().map_err(|_| Error::ParseRangeError)?,
        })
    }
}

impl FromStr for Section {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(',').ok_or(Error::ParseSectionError)?;
        Ok(Section {
            first: Range::from_str(first)?,
            second: Range::from_str(second)?,
        })
    }
}

fn extract() -> impl Iterator<Item = Section> {
    INPUT
        .lines()
        .map(|l| Section::from_str(l).expect("Failed to parse"))
}

fn is_fullcover(a: &Range, b: &Range) -> bool {
    (a.from <= b.from && a.to >= b.to) || (a.from >= b.from && a.to <= b.to)
}

fn is_overlap(a: &Range, b: &Range) -> bool {
    !(a.from > b.to || a.to < b.from)
}
