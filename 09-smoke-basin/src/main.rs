use itertools::Itertools;

use utils::aoc_problem;

type Grid = Vec<Vec<u32>>;

/// Get values of adjacent elements. Diagonals are not adjacent.
fn neighbors(g: &Grid, x: usize, y: usize) -> Vec<u32> {
    let mut out = vec![];
    if x > 0 {
        out.push(g[x - 1][y]);
    }
    if x < g.len() - 1 {
        out.push(g[x + 1][y]);
    }
    if y > 0 {
        out.push(g[x][y - 1]);
    }
    if y < g[x].len() - 1 {
        out.push(g[x][y + 1]);
    }
    out
}

fn is_low_point(g: &Grid, x: usize, y: usize) -> bool {
    neighbors(g, x, y).iter().all(|&n| n > g[x][y])
}

/// Solve the problem.
fn solve(input: &'static str) -> u32 {
    let grid: Grid = input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10))
                .map(|r| r.expect("input is valid usizes"))
                .collect()
        })
        .collect();

    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(x, y)| is_low_point(&grid, x, y))
        .map(|(x, y)| grid[x][y] + 1)
        .sum()
}

aoc_problem!(example_soln = 15);
