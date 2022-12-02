use std::collections::{HashSet, BTreeSet};
use crate::parse_data_file;

pub struct Day15 { }

impl Default for Day15 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day15 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

fn parse_input() -> anyhow::Result<Vec<Vec<u32>>> {
    // let input_string = parse_data_file("test.txt")?;
    let data = parse_data_file(super::YEAR, 15)?;
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
    let input_grid = parse_input()?;

    // Puzzle 1
    // let grid = input_grid;

    // Puzzle 2
    let mut grid = Vec::new();
    for i in 0..5 {
        for x in 0..input_grid.len() {
            let input_row = &input_grid[x];
            let mut row = Vec::new();
            for j in 0..5 {
                for y in 0..input_row.len() {
                    row.push(((input_grid[x][y] + i + j - 1) % 9) + 1);
                }
            }
            grid.push(row);
        }
    }

    let exit = (grid.len() - 1, grid[0].len() - 1);

    let mut visited = HashSet::new();
    let mut paths = BTreeSet::new();
    // Inserting the value of the path with the last location of the path.
    paths.insert((0, (0, 0)));

    while let Some((current_val, (i, j))) = paths.pop_first() {
        // found the exit!
        if (i, j) == exit {
            println!("The answer is {}", current_val);
            break;
        }

        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));

        if i > 0 {
            let c = (i - 1, j);
            paths.insert((grid[c.0][c.1] + current_val, c));
        }
        if i < grid.len() - 1 {
            let c = (i + 1, j);
            paths.insert((grid[c.0][c.1] + current_val, c));
        }
        if j > 0 {
            let c = (i, j - 1);
            paths.insert((grid[c.0][c.1] + current_val, c));
        }
        if j < grid[0].len() - 1 {
            let c = (i, j + 1);
            paths.insert((grid[c.0][c.1] + current_val, c));
        }
    }

    Ok(())
}
