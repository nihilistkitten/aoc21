use utils::aoc_problem;

const CYCLE_LEN: u8 = 7;

#[derive(Clone)]
struct Lanternfish(u8);

impl Lanternfish {
    /// Update the lanternfish, returning whether it reproduced.
    fn update(&mut self) -> bool {
        if self.0 == 0 {
            self.0 = CYCLE_LEN - 1;
            true
        } else {
            self.0 -= 1;
            false
        }
    }

    const fn new(timer: u8) -> Self {
        Self(timer)
    }

    const fn reproduce() -> Self {
        Self(CYCLE_LEN + 1)
    }
}

/// Solve the problem.
fn solve(input: &'static str) -> usize {
    let mut lanternfish: Vec<_> = input
        .trim_end() // remove trailing `\n`
        .split(',')
        .map(str::parse::<u8>)
        .map(|r| r.expect("input is valid u8s"))
        .map(Lanternfish::new)
        .collect();

    for _ in 0..80 {
        let mut count = 0;
        for fish in &mut lanternfish {
            if fish.update() {
                count += 1;
            }
        }
        lanternfish = lanternfish
            .into_iter()
            .chain(std::iter::repeat(Lanternfish::reproduce()).take(count))
            .collect();
    }

    lanternfish.len()
}

aoc_problem!(example_soln = 5934);
