use utils::aoc_problem;

fn bit_of(i: u32, bit: usize) -> u8 {
    ((i >> bit) & 1).try_into().expect("a bit is 0 or 1")
}

fn most_common_bit(inputs: &[u32], bit: usize) -> u8 {
    let mut count = 0;
    for x in inputs {
        if bit_of(*x, bit) == 1 {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count >= 0 {
        1
    } else {
        0
    }
}

fn solve(input: &'static str) -> u32 {
    let inputs: Vec<_> = input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(|s| u32::from_str_radix(s, 2).expect("input is valid binary u32s"))
        .collect();

    let max_length = input
        .split('\n')
        .next()
        .expect("at least one value in input")
        .len();

    let mut oxygen = inputs.clone();
    for bit in (0..max_length).rev() {
        let mcb = most_common_bit(&oxygen, bit);
        oxygen.retain(|n| bit_of(*n, bit) == mcb);
        if oxygen.len() == 1 {
            break;
        }
    }

    let mut co2 = inputs;
    for bit in (0..max_length).rev() {
        let lcb = 1 - most_common_bit(&co2, bit);
        co2.retain(|n| bit_of(*n, bit) == lcb);
        if co2.len() == 1 {
            break;
        }
    }

    // safety: at least one item in each vec by problem guarantees
    oxygen[0] * co2[0]
}

aoc_problem!(example_soln = 230);

/* First problem where going from part one to two was nontrivial for my solution.
 * fn solve_part_one(input: &'static str) -> u32 :
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

aoc_problem!(example_soln = 198); */
