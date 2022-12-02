use crate::parse_data_file;
use std::collections::HashSet;

pub struct Day9 {}

impl Default for Day9 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day9 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

const MAX_VAL: u32 = 10;
fn parse_input() -> anyhow::Result<Vec<Vec<u32>>> {
    let data = parse_data_file(super::YEAR, 9)?;
    let mut output = Vec::new();
    for line in data.lines() {
        let mut line_nums = Vec::new();
        for c in line.chars() {
            line_nums.push(c.to_digit(10).expect(format!("Character is not a digit! {}", c).as_str()));
        }
        output.push(line_nums);
    }
    Ok(output)
}

fn main() -> anyhow::Result<()> {
    let grid = parse_input()?;

    let mut answer = 0;
    let mut visited = HashSet::new();
    let mut basins = Vec::new();
    for i in 0..grid.len() {
        let row = &grid[i];
        for j in 0..row.len() {
            // puzzle 1
            answer += calc_lowpoint_values(i, j, &grid);

            // puzzle 2
            if !visited.contains(&(i, j)) {
                basins.push(get_basin(i, j, &grid, &mut visited));
            }
        }
    }
    println!("The answer to puzzle 1 is {}", answer);

    basins.sort();
    let mut answer_2 = 1;
    for _ in 0..3 {
        answer_2 *= basins.pop().expect("Expect to find at least three basins!");
    }
    println!("The answer to puzzle 2 is {}", answer_2);
    Ok(())
}

fn get_basin(i: usize, j: usize, grid: &Vec<Vec<u32>>, visited: &mut HashSet<(usize, usize)>) -> u32 {
    let mut stack = Vec::new();
    let mut output = 0;
    stack.push((i, j));
    while let Some((a, b)) = stack.pop() {
        if !visited.contains(&(a, b)) {
            visited.insert((a, b));
            if get_grid_value(a, b, grid) < 9 {
                output += 1;
                if a != 0 { stack.push((a - 1, b)) };
                if b != 0 { stack.push((a, b - 1)) };
                stack.push((a + 1, b));
                stack.push((a, b + 1));
            }
        }
    }
    output
}

fn calc_lowpoint_values(i: usize, j: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let val = get_grid_value(i, j, &grid);
    let up = if i == 0 { 10 } else { get_grid_value(i - 1, j, &grid) };
    let down = get_grid_value(i + 1, j, &grid);
    let left = if j == 0 { 10 } else { get_grid_value(i, j - 1, &grid) };
    let right = get_grid_value(i, j + 1, &grid);

    // low point
    if val < up && val < down && val < left && val < right {
        val + 1
    } else {
        0
    }
}

fn get_grid_value(i: usize, j: usize, grid: &Vec<Vec<u32>>) -> u32 {
    if i >= grid.len() || ( i < grid.len() && j >= grid[i].len() ) {
        MAX_VAL
    } else {
        grid[i][j]
    }
}
