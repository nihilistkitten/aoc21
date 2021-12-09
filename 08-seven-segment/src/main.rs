#![feature(map_try_insert)]

use std::collections::{BTreeSet, HashMap, HashSet};

use utils::aoc_problem;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Wire {
    fn from_char(c: char) -> Self {
        match c {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => panic!("invalid char: {}", c),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Signal(BTreeSet<Wire>);

fn parse_signals<C: FromIterator<Signal>>(v: &str) -> C {
    v.trim()
        .split(' ')
        .map(|s| Signal(s.chars().map(Wire::from_char).collect()))
        .collect()
}

fn solve_line(mut left: HashSet<Signal>, right: Vec<Signal>) -> u32 {
    let one = left.iter().find(|s| s.0.len() == 2).unwrap().clone();
    left.remove(&one);

    let four = left.iter().find(|s| s.0.len() == 4).unwrap().clone();
    left.remove(&four);

    let seven = left.iter().find(|s| s.0.len() == 3).unwrap().clone();
    left.remove(&seven);

    let eight = left.iter().find(|s| s.0.len() == 7).unwrap().clone();
    left.remove(&eight);

    let nine = left
        .iter()
        .find(|s| s.0.len() == 6 && s.0.is_superset(&four.0))
        .unwrap()
        .clone();
    left.remove(&nine);

    let three = left
        .iter()
        .find(|s| s.0.len() == 5 && s.0.is_superset(&one.0))
        .unwrap()
        .clone();
    left.remove(&three);

    let zero = left
        .iter()
        .find(|s| s.0.len() == 6 && s.0.is_superset(&one.0))
        .unwrap()
        .clone();
    left.remove(&zero);

    let six = left.iter().find(|s| s.0.len() == 6).unwrap().clone();
    left.remove(&six);

    let five = left
        .iter()
        .find(|s| s.0.len() == 5 && s.0.is_subset(&six.0))
        .unwrap()
        .clone();
    left.remove(&five);

    let two = left.iter().find(|s| s.0.len() == 5).unwrap().clone();
    left.remove(&two);

    let map = HashMap::from([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ]);

    right
        .iter()
        .map(|s| map.get(s).unwrap())
        .fold(0, |s, n| s * 10 + n)
}

fn solve(input: &'static str) -> u32 {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(|s| s.split('|'))
        .map(|mut line| -> u32 {
            solve_line(
                parse_signals(line.next().expect("input is well-formed")),
                parse_signals(line.next().expect("input is well-formed")),
            )
        })
        .sum::<u32>()
}

aoc_problem!(example_soln = 61229);
