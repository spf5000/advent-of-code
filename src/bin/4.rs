use std::collections::HashSet;

use advent_of_code::parse_data_file;

// Number of rows/columns in the grid
const GRID_SIZE: usize = 5;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<GridCell>>,
    won: bool
}

impl Grid {
    fn new(grid: Vec<Vec<GridCell>>) -> Self {
        Self {
            grid,
            won: false
        }
    }
}

#[derive(Debug)]
struct Input {
    nums: Vec<u32>,
    grids: Vec<Grid>
}

#[derive(Debug)]
struct GridCell {
    num: u32,
    found: bool
}

impl GridCell {
    fn new(num: &str) -> Self {
        Self {
            num: num.parse().expect("Parsing non-numeric str in the grid"),
            found: false
        }
    }
}

fn parse_input() -> anyhow::Result<Input> {
    // let input_string = parse_data_file("test.txt")?;
    let input_string = parse_data_file("4.txt")?;
    let mut line_iter = input_string.lines().peekable();

    // first line is the numbers
    let numbers_str = line_iter.next().expect("Could not get numbers out of the input!");

    // next there is a blank line
    line_iter.next().expect("Expecting an empty line between the numbers and the grids!");

    // assuming if there is a next line
    let mut grids = Vec::new();
    while line_iter.peek().is_some() {
        let mut grid = Vec::new();
        for i in 0..GRID_SIZE {
            let grid_row_str = line_iter.next()
                .expect(format!("Expecting {} row of grid in input", i).as_str());
            let grid_row = grid_row_str.split(' ')
                .filter(|cell| !cell.is_empty())
                .map(|cell| GridCell::new(cell))
                .collect();
            grid.push(grid_row);
        }
        grids.push(Grid::new(grid));

        // there is one blank line between grids
        line_iter.next().expect("Expecting a blank line between the grids!");
    }

    Ok(Input {
        nums: numbers_str.split(',')
            .map(|num| num.parse().expect("Non-numeric string in the numbers list!"))
            .collect(),
        grids
    })
}

fn call_number_for_grid(num: u32, grid: &mut Grid) -> bool {
    let mut found_rows = HashSet::new();
    let mut found_cols = HashSet::new();
    for i in 0..grid.grid.len() {
        let row = &mut grid.grid[i];
        for j in 0..row.len() {
            if row[j].num == num {
                row[j].found = true;
                found_rows.insert(i);
                found_cols.insert(j);
            }
        }
    }

    // check row
    for i in found_rows {
        let mut row_bingo = true;
        for grid_cell in &grid.grid[i] {
            row_bingo = row_bingo && grid_cell.found;
        }
        if row_bingo { return true }
    }

    // check colums
    for j in found_cols {
        let mut col_bingo = true;
        for row in grid.grid.iter() {
            col_bingo = col_bingo && row[j].found;
        }
        if col_bingo { return true }
    }

    false
}

fn sum_grid(grid: &Grid) -> u32 {
    let mut output = 0;
    for row in &grid.grid {
        for cell in row {
            if !cell.found {
                output += cell.num;
            }
        }
    }
    output
}

fn main() -> anyhow::Result<()> {
    let mut input = parse_input()?;
    for num in input.nums {
        for mut grid in &mut input.grids {
            // Round 1: First winner calculated.
            if call_number_for_grid(num, &mut grid) {
                let grid_sum = sum_grid(&grid);
                println!("The Answer: {}", grid_sum * num);
                return Ok(());
            }

            // Round 2: skip if this grid has already won.
            // if !grid.won && call_number_for_grid(num, &mut grid) {
            //     grid.won = true;
            //     let grid_sum = sum_grid(&grid);
            //     println!("The Answer: {}", grid_sum * num);
            // }
        }
    }

    Ok(())
}
