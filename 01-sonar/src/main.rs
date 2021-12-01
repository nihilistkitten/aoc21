use itertools::Itertools;

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
