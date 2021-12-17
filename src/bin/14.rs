use std::collections::{HashMap, BTreeMap};

use advent_of_code::parse_data_file;
use anyhow::anyhow;

const STEPS: u32 = 40;

#[derive(Debug, Clone)]
struct Input<'a> {
    template: String,
    insertions: HashMap<&'a str, char>
}

impl <'a> Input<'a> {
    fn new(template: String, insertions: HashMap<&'a str, char>) -> Self {
        Self { template, insertions }
    }
}

fn parse_input<'a>(input_string: &'a String) -> anyhow::Result<Input<'a>> {
    let mut lines = input_string.lines();
    let template = lines.next().ok_or(anyhow!("Expected input to start with a template line!"))?;
    lines.next().ok_or(anyhow!("Expected input to have empty line after template!"))?;
    let insertions = lines.map(|line| {
        let mut split = line.split(" -> ");
        let left = split.next().expect(&format!("Expected insertions on line with left: {}", line));
        let right = split.next().expect(&format!("Expected insertions on line with right: {}", line));
        (left, right.chars().next().unwrap())
    }).collect();

    Ok(Input::new(String::from(template), insertions))
}

fn main() -> anyhow::Result<()> {
    // let input_string = parse_data_file("test.txt")?;
    let input_string = parse_data_file("14.txt")?;
    let mut input = parse_input(&input_string)?;
    let mut char_count_map = BTreeMap::new();
    for step in 0..STEPS {
        let mut next_template = String::new();
        let mut template_iter = input.template.chars().peekable();

        // the first character will always be in the new template
        next_template.push(*template_iter.peek()
                           .ok_or(anyhow!("Expected {} to have at least one character!", input.template))?);
        while let Some(first_char) = template_iter.next() {
            if let Some(second_char) = template_iter.peek() {
                let mut key = first_char.to_string();
                key.push(*second_char);
                if let Some(extra) = input.insertions.get(key.as_str()) {
                    next_template.push(*extra);
                    if step == STEPS - 1 {
                        update_map(&mut char_count_map, *extra);
                    }
                }
                next_template.push(*second_char);
                if step == STEPS - 1 {
                    update_map(&mut char_count_map, *second_char);
                }
            }
        }
        input.template = next_template;
        println!("Template len after {} steps: {}", step, input.template.len());
    }

    let mut min = None;
    let mut max = None;
    for (_, val) in char_count_map {
        if min.is_none() || min.unwrap() > val {
            min = Some(val);
        }

        if max.is_none() || max.unwrap() < val {
            max = Some(val);
        }
    }
    
    println!("Min: {:?}, Max: {:?}", min, max);
    println!("Puzzle 1 answer is {}", max.unwrap() - min.unwrap());

    Ok(())
}

fn update_map(map: &mut BTreeMap<char, u64>, c: char) {
    let mut count = map.entry(c).or_insert(0);
    *count += 1;
}
