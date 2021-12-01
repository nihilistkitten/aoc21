/// Read the example input from `resources/example.txt`.
#[macro_export]
macro_rules! example_input {
    () => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/example.txt"
        ))
    };
}

/// Read the example input from `resources/input.txt`.
#[macro_export]
macro_rules! problem_input {
    () => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/input.txt"))
    };
}

/// Generate a main function which relies on `solve` and a test of the example input.
#[macro_export]
macro_rules! aoc_problem {
    (example_soln = $out:expr) => {
        use utils::problem_input;

        fn main() {
            println!("{}", solve(problem_input!()));
        }

        #[cfg(test)]
        mod generated_tests {
            use super::*;
            use utils::example_input;

            #[test]
            fn test_example() {
                assert_eq!(solve(example_input!()), $out);
            }
        }
    };
}
