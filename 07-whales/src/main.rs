use utils::aoc_problem;

/// Solve the problem.
fn solve(input: &'static str) -> u32 {
    let initial_positions = input
        .trim_end() // remove trailing `\n`
        .split(',')
        .map(str::parse::<u32>)
        .map(|r| r.expect("input is valid u32s"))
        .collect::<Vec<_>>();

    let &min = initial_positions.iter().min().expect("numbers in input");
    let &max = initial_positions.iter().max().expect("numbers in input");

    (min..=max)
        .map(|n| {
            initial_positions
                .iter()
                .map(|&m| if n > m { n - m } else { m - n })
                .sum()
        })
        .min()
        .expect("numbers in input")
}

aoc_problem!(example_soln = 37);
