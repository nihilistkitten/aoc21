use itertools::Itertools;

/// Preprocess the input string into an iterator of usizes.
fn preprocess_input(input: &'static str) -> impl Iterator<Item = usize> {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(str::parse)
        .map(|r| r.expect("input is valid usizes"))
}

/// Group depths by sum of rolling windows of three.
fn group_by_three(depths: impl Iterator<Item = usize>) -> impl Iterator<Item = usize> {
    depths.tuple_windows().map(|(p, c, n)| p + c + n)
}

/// Compute the number of times the sequence increases.
fn compute_increases(depths: impl Iterator<Item = usize>) -> usize {
    depths
        .tuple_windows()
        .fold(0, |t, (p, c)| if c > p { t + 1 } else { t })
}

/// Solve the problem.
fn solve(input: &'static str) -> usize {
    compute_increases(group_by_three(preprocess_input(input)))
}

fn main() {
    println!(
        "{}",
        solve(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/input.txt"
        )))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            solve(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/example.txt"
            ))),
            5
        );
    }
}
