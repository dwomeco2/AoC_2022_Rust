
const INPUT: &str = include_str!("../../input/day13");

use std::cmp::Ordering;

use itertools::Itertools;
use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "src/days/day13.pest"]
pub struct NestedListParser;

pub fn part1() {
    let pairs = parse();
    
    let result = pairs.iter().enumerate().filter_map(|(index, p)| {
        match compare_pair(p) {
            Ordering::Less | Ordering::Equal => Some(index + 1),
            Ordering::Greater => None
        }
    }).sum::<usize>();

    println!("{result}");
}

pub fn part2() {
    let mut pairs = parse();
    let d1 = Value::Arr(vec![Value::Num(2)]);
    let d2 = Value::Arr(vec![Value::Num(6)]);
    pairs.push((d1.clone(), d2.clone()));
    let (mut a1, mut a2): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    a1.append(&mut a2);
    a1.sort();
    let result = a1.into_iter().enumerate()
    .filter_map(|(index, d)| match d == d1 || d == d2 {
        true => Some(index + 1),
        false => None
    }).product::<usize>();
    println!("{result}");
}

fn compare_pair(pair: &PacketPair) -> Ordering {
    let (p0, p1) = pair;
    p0.cmp(p1)
}

fn parse() -> Vec<PacketPair> {
    let file = NestedListParser::parse(Rule::file, INPUT)
    .expect("parse error");

    let mut pairs = vec![];
    for t in file {
        for r in t.into_inner() {
            if r.as_rule() != Rule::EOI {
                let p = parse_pair(r);
                pairs.push(p);
            }
        }
    }
    pairs
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Arr(Vec<Value>),
    Num(usize),
}

type PacketPair = (Value, Value);

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Num(_), Value::Arr(_)) => {
                Value::Arr(vec![self.clone()]).cmp(other)
            }
            (Value::Arr(_), Value::Num(_)) => {
                self.cmp(&Value::Arr(vec![other.clone()]))
            }
            (Value::Num(n1), Value::Num(n2)) => n1.cmp(n2),
            (Value::Arr(a1), Value::Arr(a2)) => {
                for i in 0.. {
                    let (a1, a2) = (a1.get(i), a2.get(i));
                    if a1.is_none() && a2.is_none() {
                        return Ordering::Equal;
                    }
                    let result = a1.cmp(&a2);
                    if a1.cmp(&a2) != Ordering::Equal {
                        return result;
                    }
                }
                unreachable!()
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_pair(rule: Pair<Rule>) -> PacketPair {
    rule.into_inner().map(Value::parse_arr).collect_tuple::<PacketPair>().unwrap()
}

impl Value {
    pub fn parse(rule: Pair<Rule>) -> Vec<Value> {
        rule.into_inner().filter_map(|r| match r.as_rule() {
            Rule::array => Some(Value::parse_arr(r)),
            Rule::num => Some(Value::parse_num(r)),
            _ => None,
        }).collect::<Vec<_>>()
    }

    pub fn parse_arr(rule: Pair<Rule>) -> Value {
        let rs = rule.into_inner();
        let v = rs.flat_map(Value::parse).collect::<Vec<_>>();
        Value::Arr(v)
    }

    pub fn parse_num(rule: Pair<Rule>) -> Value {
        Value::Num(rule.as_str().parse::<usize>().unwrap())
    }
}