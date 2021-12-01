use itertools::Itertools;
use utils::aoc_problem;

/// Solve the problem.
fn solve(input: &'static str) -> usize {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(str::parse::<usize>)
        .map(|r| r.expect("input is valid usizes"))
        .tuple_windows()
        .map(|(p, c, n)| p + c + n)
        .tuple_windows()
        .fold(0, |t, (p, c)| if c > p { t + 1 } else { t })
}

aoc_problem!(example_soln = 5);
