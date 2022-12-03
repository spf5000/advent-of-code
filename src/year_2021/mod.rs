// Problems define functions by day. Didn't organize this as well in 2021, but should be more
// consistent in 2022. also commenting out days that weren't completed, but should exist so this
// can be re-used this year.

use crate::model::Year;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
// mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
// mod day_19;
mod day_20;
mod day_21;
// mod day_22;
// mod day_23;
// mod day_24;
// mod day_25;

const YEAR: Year = Year::Year2021;

pub struct Year2021 { }

impl Default for Year2021 {
    fn default() -> Self {
        Self {}
    }
}

impl crate::YearAnswers for Year2021 {
    fn get_answers_for_day(&self, day: u8) -> anyhow::Result<Box<dyn crate::DayAnswers>> {
        match day {
            1 => Ok(Box::new(day_1::Day1::default())),
            2 => Ok(Box::new(day_2::Day2::default())),
            3 => Ok(Box::new(day_3::Day3::default())),
            4 => Ok(Box::new(day_4::Day4::default())),
            5 => Ok(Box::new(day_5::Day5::default())),
            6 => Ok(Box::new(day_6::Day6::default())),
            7 => Ok(Box::new(day_7::Day7::default())),
            8 => Ok(Box::new(day_8::Day8::default())),
            9 => Ok(Box::new(day_9::Day9::default())),
            10 => Err(anyhow::anyhow!("Year 2021, Day 10 wasn't completed :(")),
            11 => Ok(Box::new(day_11::Day11::default())),
            12 => Ok(Box::new(day_12::Day12::default())),
            13 => Ok(Box::new(day_13::Day13::default())),
            14 => Ok(Box::new(day_14::Day14::default())),
            15 => Ok(Box::new(day_15::Day15::default())),
            16 => Ok(Box::new(day_16::Day16::default())),
            17 => Ok(Box::new(day_17::Day17::default())),
            18 => Ok(Box::new(day_18::Day18::default())),
            19 => Err(anyhow::anyhow!("Year 2021, Day 19 wasn't completed :(")),
            20 => Ok(Box::new(day_20::Day20::default())),
            21 => Ok(Box::new(day_21::Day21::default())),
            _ => Err(anyhow::anyhow!("Invalid day provided! The day must be between the 1st and 25th (of December)! {}", day))
        }
    }
}

