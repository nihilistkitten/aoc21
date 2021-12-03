use std::collections::HashMap;

use utils::aoc_problem;

fn solve(input: &'static str) -> u32 {
    let val = input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .flat_map(|s| {
            s.chars()
                .map(|c| c.to_digit(2).expect("input is valid binary"))
                .rev()
                .enumerate()
                .map(|(i, v)| (i.try_into().expect("less than u32::max digits"), v))
        })
        .fold(HashMap::default(), |mut hm: HashMap<u32, _>, (i, val)| {
            let entry = hm.entry(i).or_insert(0);
            if val == 1 {
                *entry += 1;
            } else {
                *entry -= 1;
            };
            hm
        })
        .iter()
        .fold((0, 0), |(gamma, epsilon), (index, count)| {
            if *count >= 0 {
                (gamma + (1 << index), epsilon)
            } else {
                (gamma, epsilon + (1 << index))
            }
        });

    val.0 * val.1
}

aoc_problem!(example_soln = 198);
