use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, one_of},
    multi::separated_list1,
    sequence::{delimited, pair, preceded},
    IResult,
};

const INPUT: &str = include_str!("../../input/day11");

pub fn part1() {
    let mut monkeys = parse();
    let result = find_monkey_business(&mut monkeys, 20, false);
    println!("{result}");
}

pub fn part2() {
    let mut monkeys = parse();
    let result = find_monkey_business(&mut monkeys, 10000, true);
    println!("{result}");
}

fn find_monkey_business(monkeys: &mut Vec<Monkey>, round: usize, unmanagable: bool) -> usize {

    let div_mod = monkeys.iter().map(|m| m.div_val).product::<usize>();

    for _round in 1..=round {
        for index in 0..monkeys.len() {
            
            {
                let monkey = &mut monkeys[index];
                monkey.update_inspects_count();
            }

            let items = monkeys[index].items.clone();

            for mut item in items {
                item = (monkeys[index].operation)(item);
                if unmanagable {
                    item %= div_mod;
                } else {
                    item /= 3;
                }
                let throw_to = (monkeys[index].throw_route)(item);
                monkeys[throw_to].catch(item);
            }

            monkeys[index].items.clear();
        }
    }
    
    let abc = monkeys.iter().map(|x| x.inspect_count).sorted().collect_vec();
    abc.last().unwrap() * abc[abc.len() - 2]
}

fn parse<'a>() -> Vec<Monkey<'a>> {
    INPUT
        .lines()
        .chunks(7)
        .into_iter()
        .map(|chunk| {
            let it = chunk.skip(1).map(|l| l.split_once(':').map(|x| x.1).unwrap());
            Monkey::parse(it)
        })
        .collect()
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

type MonkeyIndex = usize;
type WorryLevel = usize;
type Throw = MonkeyIndex;

struct Monkey<'a> {
    items: Vec<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel + 'a>,
    throw_route: Box<dyn Fn(WorryLevel) -> Throw + 'a>,
    inspect_count: usize,
    div_val: usize,
}

impl<'a> Monkey<'a> {
    pub fn parse(mut it: impl Iterator<Item = &'a str>) -> Self {
        let (_, items_parse) = separated_list1(tag(", "), ws(digit1))(it.next().unwrap()).unwrap();
        let (_, (op, op_val)) = preceded(
            ws(tag("new = old")),
            pair(one_of("+*"), alt((ws(digit1), ws(tag("old"))))),
        )(it.next().unwrap())
        .unwrap();
        let (_, div_val) = preceded(ws(tag("divisible by")), digit1)(it.next().unwrap()).unwrap();
        let (_, true_val) =
            preceded(ws(tag("throw to monkey")), digit1)(it.next().unwrap()).unwrap();
        let (_, false_val) =
            preceded(ws(tag("throw to monkey")), digit1)(it.next().unwrap()).unwrap();

        let items = items_parse.iter().map(|&s| s.parse::<usize>().unwrap()).collect();
        let operation = move |worry_level: WorryLevel| -> WorryLevel {
            match op {
                '+' => worry_level + op_val.parse::<usize>().unwrap_or(worry_level),
                '*' => worry_level * op_val.parse::<usize>().unwrap_or(worry_level),
                _ => unreachable!(),
            }
        };

        let div_val = div_val.parse::<usize>().unwrap();

        let throw_route = move |worry_level: WorryLevel| -> Throw {
            if worry_level % div_val == 0 {
                true_val.parse::<usize>().unwrap()
            } else {
                false_val.parse::<usize>().unwrap()
            }
        };

        Self { items, operation: Box::new(operation), throw_route: Box::new(throw_route), inspect_count: 0, div_val }
    }

    fn catch(&mut self, item: WorryLevel) {
        self.items.push(item);
    }

    fn update_inspects_count(&mut self) {
        self.inspect_count += self.items.len();
    }
}
