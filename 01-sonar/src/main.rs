/// Preprocess the input string into an iterator of usizes.
fn preprocess_input(input: &'static str) -> impl Iterator<Item = usize> {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(str::parse)
        .map(|r| r.expect("input is valid usizes"))
}

/// Compute the number of times the sequence increases.
fn compute_increases(depths: impl Iterator<Item = usize>) -> usize {
    let mut prev = None;
    let mut res = 0;
    for curr in depths {
        if let Some(prev_depth) = prev {
            if curr > prev_depth {
                res += 1;
            }
        }
        prev = Some(curr);
    }
    res
}

fn main() {
    println!(
        "{}",
        compute_increases(preprocess_input(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/input.txt"
        ))))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            compute_increases(preprocess_input(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/example.txt"
            )))),
            7
        );
    }
}
