use std::{collections::{HashMap}, ops::Add, io::BufRead};
use anyhow::anyhow;

use advent_of_code::parse_data_file;

const DAYS: u16 = 256;

fn main() -> anyhow::Result<()> {
    let input_string = parse_data_file("6.txt")?;
    // let input_string = parse_data_file("test.txt")?;
    let fishes: Vec<usize> = input_string.split(',').into_iter()
        .map(|num_str| num_str.trim().parse().expect(&format!("Expecting a number from input string! {}", num_str)))
        .collect();

    let mut fish_counts: [u128; 9] = [0; 9];
    for fish in fishes {
        fish_counts[fish] += 1;
    }

    for _ in 0..DAYS {
        let mut previous = fish_counts[fish_counts.len() - 1];
        // decrement all but the zeros
        for i in 2..=fish_counts.len() {
            let new_previous = fish_counts[fish_counts.len() - i];
            fish_counts[fish_counts.len() - i] = previous;
            previous = new_previous;
        }
        // handle the 0's
        fish_counts[6] += previous;
        fish_counts[8] = previous;
    }

    let answer: u128 = fish_counts.into_iter().fold(0, |acc, count| acc + count as u128);
    println!("The answer is {}", answer);

    Ok(())
}

