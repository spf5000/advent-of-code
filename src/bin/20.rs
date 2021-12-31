use advent_of_code::parse_data_file;
use anyhow::anyhow;

const PART_1: u8 = 2;
const PART_2: u8 = 50;

struct Input {
    algorithm: Vec<bool>,
    grid: Vec<Vec<bool>>
}

fn parse_input() -> anyhow::Result<Input> {
    let input_string = parse_data_file("test.txt")?;
    // let input_string = parse_data_file("20.txt")?;
    let mut input_iter = input_string.lines();
    let algorithm = parse_line(input_iter.next().expect("Expect the first line to be the algorithm"));
    // skip empty line. 
    input_iter.next();
    let mut grid = Vec::new();
    while let Some(row) = input_iter.next() {
        grid.push(parse_line(row));
    }

    Ok(Input { algorithm, grid })
}

fn parse_line<S: AsRef<str>>(line: S) -> Vec<bool> {
    line.as_ref().chars().map(|c| match c {
        '#' => true,
        '.' => false,
        _ => panic!("Unexpected character in input! {}", c)
    }).collect()
}

fn enhance_image(grid: Vec<Vec<bool>>, algorithm: &Vec<bool>, step: u8) -> anyhow::Result<Vec<Vec<bool>>> {
    let is_outer_lit = step % 2 != 0;
    // let is_outer_lit = false;
    let mut output = Vec::new();
    for i in -1..=grid.len() as i32 {
        let mut output_row = Vec::new();
        let prev_row = grid.get((i-1) as usize);
        let current_row = grid.get(i as usize);
        let next_row = grid.get((i+1) as usize);
        let row_len = match (prev_row, current_row, next_row) {
            (Some(row), _, _) => row.len(),
            (None, Some(row), _) => row.len(),
            (None, None, Some(row)) => row.len(),
            (None, None, None) => return Err(anyhow!("At least one row should be accessible at row {}", i))
        } as i32;
        for j in -1..=row_len {
            let mut num: u16 = 0;
            // num += prev_row.map(|row| read_bits(row, j, is_outer_lit)).unwrap_or(0);
            num += prev_row.map(|row| read_bits(row, j, is_outer_lit)).unwrap_or(if is_outer_lit {7} else {0});
            num <<= 3;
            // num += current_row.map(|row| read_bits(row, j, is_outer_lit)).unwrap_or(0);
            num += current_row.map(|row| read_bits(row, j, is_outer_lit)).unwrap_or(if is_outer_lit {7} else {0});
            num <<= 3;
            // num += next_row.map(|row| read_bits(row, j, is_outer_lit)).unwrap_or(0);
            num += next_row.map(|row| read_bits(row, j, is_outer_lit)).unwrap_or(if is_outer_lit {7} else {0});
            output_row.push(*algorithm.get(num as usize).ok_or(anyhow!("Expected number to be in the algorith vector! {}", num))?);
        }
        output.push(output_row);
    }

    Ok(output)
}

fn read_bits(row: &Vec<bool>, i: i32, default: bool) -> u16 {
    let mut output = 0;
    // First digit of the three digit binary. Maps to 0 or 4.
    if *row.get((i - 1) as usize).unwrap_or(&default) { output += 4 }
    // Middle digit of the three digit binary. Maps to 0 or 2.
    if *row.get(i as usize).unwrap_or(&default) { output += 2 }
    // Last digit of the three digit binary. Maps to 0 or 1.
    if *row.get((i + 1) as usize).unwrap_or(&default) { output += 1 }
    output
}

fn get_lit_count(grid: &Vec<Vec<bool>>) -> u32 {
    grid.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc, lit| {
        if *lit { acc + 1 } else { acc }
    }))
}

fn main() -> anyhow::Result<()> {
    let input = parse_input()?;
    let mut grid = input.grid;

    let mut part_1 = 0;
    for step in 0..PART_2 {
        if step <= PART_1 {
            println!("\nGrid: ");
            for row in grid.iter() {
                println!("{}", row.iter().map(|b| if *b {'#'} else {'.'}).collect::<String>());
            }
        }
        grid = enhance_image(grid, &input.algorithm, step)?;
        if step + 1 == PART_1 {
            part_1 = get_lit_count(&grid);
        }
    }

    println!("The Answer to part 1 is {}", part_1);
    println!("The Answer to part 2 is {}", get_lit_count(&grid));

    Ok(())
}
