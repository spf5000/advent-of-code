use std::{str::FromStr, collections::HashSet};

use crate::parse_data_file;
use anyhow::anyhow;

pub struct Day13 { }

impl Default for Day13 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day13 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

#[derive(Debug, Clone)]
struct Input {
    coordinates: HashSet<(u32, u32)>,
    folds: Vec<Fold>
}

#[derive(Debug, Clone)]
enum FoldType {
    Vertical,
    Horizontal
}

impl FromStr for FoldType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::Vertical),
            "y" => Ok(Self::Horizontal),
            other => Err(anyhow!("{} is not x or y!", other))
        }
    }
}

#[derive(Debug, Clone)]
struct Fold {
    fold_type: FoldType,
    location: u32
}

fn parse_input() -> anyhow::Result<Input> {
    // let input_string = parse_data_file("test.txt")?;
    let data = parse_data_file(super::YEAR, 13)?;
    let mut lines = data.lines();
    let mut coordinates = HashSet::new();
    // parse the coordiantes
    while let Some(line) = lines.next() {
        // break at the empty line, then we'll need to handle the folds
        if line.trim().is_empty() {
            break;
        }

        let mut split = line.split(',');
        let x = split.next().ok_or(anyhow!("No x in coordinate!"))?;
        let y = split.next().ok_or(anyhow!("No y in coordinate!"))?;
        coordinates.insert((x.parse()?, y.parse()?));
    }

    let mut folds = Vec::new();
    while let Some(line) = lines.next() {
        let fold_info = &line["fold along ".len()..];
        let mut split = fold_info.split('=');
        let fold_type = split.next().ok_or(anyhow!("Expecting {} to have an along", fold_info))?;
        let location = split.next().ok_or(anyhow!("Expecting {} to have a location", fold_info))?;
        folds.push(Fold { fold_type: FoldType::from_str(fold_type)?, location: location.parse()? });
    }

    Ok(Input { coordinates, folds })
}

fn main() -> anyhow::Result<()> {
    let input = parse_input()?;
    let mut coordinates = input.coordinates;
    let mut first_answer = None;
    for fold in input.folds {
        coordinates = coordinates.into_iter().map(|(x, y)| {
            match (x > fold.location, y > fold.location, &fold.fold_type) {
                (true, _, FoldType::Vertical) => (get_new_location(x, fold.location), y),
                (_, true, FoldType::Horizontal) => (x, get_new_location(y, fold.location)),
                (_, _, _) => (x, y)
            }
        }).collect();

        // Get the coordinate count after the first day
        if first_answer.is_none() {
            first_answer = Some(coordinates.len());
        }
    }

    println!("The answer to puzzle 1 is {:?}", first_answer);

    // build and print the grid
    let mut grid = Vec::new();
    for (x, y) in coordinates {
        if grid.len() as u32 <= y {
            let index: usize = (y + 1).try_into()?;
            grid.resize(index, Vec::new());
        }
        let row = &mut grid[y as usize];
        if row.len() as u32 <= x {
            let index: usize = (x + 1).try_into()?;
            row.resize(index, '.');
        }
        row[x as usize] = '#';
    }

    println!("The answer to puzzle 2 is:");
    for row in grid {
        println!("\t{:?}", row);
    }
    Ok(())
}

fn get_new_location(original_location: u32, fold_location: u32) -> u32 {
    if original_location <= fold_location {
        panic!("Original location {} is less than or equal to the fold location {}!", original_location, fold_location);
    }

    let delta = original_location - fold_location;
    fold_location - delta
}
