use advent_of_code::parse_data_file;

use std::collections::HashSet;
use std::ops::Range;

const STEPS: u32 = 100;
const FLASH_THRESHOLD: u32 = 10;

fn parse_input() -> anyhow::Result<Vec<Vec<u32>>> {
    let input_string = parse_data_file("11.txt")?;
    // let input_string = parse_data_file("test.txt")?;
    let mut output = Vec::new();
    for line in input_string.lines() {
        let mut line_nums = Vec::new();
        for c in line.chars() {
            line_nums.push(c.to_digit(10).expect(format!("Character is not a digit! {}", c).as_str()));
        }
        output.push(line_nums);
    }
    Ok(output)
}

fn main() -> anyhow::Result<()> {
    let mut grid = parse_input()?;
    let cell_count = grid.iter().fold(0, |acc, row| acc + row.len());
    let mut flashes = Vec::new();
    let mut flashed = HashSet::new();
    let mut answer = 0;
    // for step in 0..STEPS {
    for step in 1.. {
        let mut flash_count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if update_grid_value(i, j, &mut grid, &flashed) {
                    flashes.push((i, j));
                    flashed.insert((i, j));
                    flash_count += 1;
                }
            }
        }

        // handle the flashes.
        while let Some((a, b)) = flashes.pop() {
        // while let Some((a, b)) = flashes.pop() {
            for x in 0..=2 {
                for y in 0..=2 {
                    if a + x == 0 || b + y == 0 {
                        continue;
                    }

                    let adjacent = (a + x - 1, b + y - 1);
                    if update_grid_value(adjacent.0, adjacent.1, &mut grid, &flashed) {
                        flashes.push(adjacent);
                        flashed.insert(adjacent);
                        flash_count += 1;
                    }
                }
            }
        }

        for (a, b) in flashed.iter() {
            grid[*a][*b] = 0;
        }
        flashes.truncate(0);
        if flash_count == cell_count {
            println!("The answer to puzzle 2 is {}", step);
            break;
        }
        flashed.drain();
        answer += flash_count;
    }

    println!("The answer to puzzle 1 is {}", answer);
    Ok(())
}

fn update_grid_value(i: usize, 
                     j: usize, 
                     grid: &mut Vec<Vec<u32>>, 
                     flashed: &HashSet<(usize, usize)>) -> bool {
    if flashed.contains(&(i, j)) || i >= grid.len() || ( i < grid.len() && j >= grid[i].len() ) {
        false
    } else {
        grid[i][j] += 1;
        grid[i][j] >= FLASH_THRESHOLD
    }
}
