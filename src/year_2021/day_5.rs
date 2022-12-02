use std::collections::HashMap;
use anyhow::anyhow;

use crate::parse_data_file;

pub struct Day5 {}

impl Default for Day5 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day5 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn to_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

fn parse_input() -> anyhow::Result<Vec<(Point, Point)>> {
    let data = parse_data_file(super::YEAR, 5)?;
    // let input_string = parse_data_file("test.txt")?;
    let mut output = Vec::new();
    for line in data.lines() {
        let mut split = line.split(" -> ");
        let from = split.next().ok_or(anyhow!("Couldn't find 'from' in line split!: {}", &line))?;
        let to = split.next().ok_or(anyhow!("Couldn't find 'to' in line split!: {}", &line))?;
        output.push((str_to_point(from)?, str_to_point(to)?));
    }
    Ok(output)
}

fn str_to_point<S: AsRef<str>>(input: S) -> anyhow::Result<Point> {
    let mut split = input.as_ref().split(',');
    let x = split.next().ok_or(anyhow!("Couldn't find 'x' in coordinate split!: {}", &input.as_ref()))?;
    let y = split.next().ok_or(anyhow!("Couldn't find 'y' in coordinate split!: {}", &input.as_ref()))?;
    Ok(Point { x: x.parse()?, y: y.parse()? })
}

struct ReverseRangeInclusive {
    end: i64,
    current: i64,
    is_backwards: bool
}

impl ReverseRangeInclusive {
    fn new(start: i64, end_inclusive: i64) -> Self {
        Self {
            current: start,
            end: end_inclusive,
            is_backwards: end_inclusive < start
        }
    }
}

impl Iterator for ReverseRangeInclusive {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_backwards {
            if self.current < self.end { None } else {
                let output = Some(self.current);
                self.current -= 1;
                output
            }
        } else {
            if self.current > self.end { None } else {
                let output = Some(self.current);
                self.current += 1;
                output
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = parse_input()?;
    let mut point_count_map = HashMap::new();

    for i in 9..=0 {
        println!("I: {}", i);
    }

    // Count points
    for (from, to) in input {
        match (from.x == to.x, from.y == to.y) {
            // same on x axis
            (true, false) => {
                let range = if from.y < to.y { from.y..=to.y } else { to.y..=from.y };
                for y in range {
                    *point_count_map.entry((from.x, y)).or_insert(0) += 1;
                }
            },
            // same on y axis
            (false, true) => {
                let range = if from.x < to.x { from.x..=to.x } else { to.x..=from.x };
                for x in range {
                    *point_count_map.entry((x, from.y)).or_insert(0) += 1;
                }
            }

            // diagonal hopefully :)
            (false, false) => {
                if (to.x - from.x).abs() != (to.y - from.y).abs() {
                    return Err(anyhow!("Diagonal not at 45 degrees!: {:?}, {:?}", from.to_tuple(), to.to_tuple()));
                }
                let x_range = ReverseRangeInclusive::new(from.x, to.x);
                let y_range = ReverseRangeInclusive::new(from.y, to.y);
                for coordinate in x_range.zip(y_range) {
                    *point_count_map.entry(coordinate).or_insert(0) += 1;
                }
            },

            (true, true) => { *point_count_map.entry(from.to_tuple()).or_insert(0) += 1 }
        }
    }

    let answer = point_count_map.into_values().fold(0, |acc, val| {
        if val > 1 { acc + 1 } else { acc }
    });

    println!("The answer is {}!", answer);

    Ok(())
}
