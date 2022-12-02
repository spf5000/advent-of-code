use std::rc::Rc;
use std::cell::RefCell;
use std::convert::From;
use anyhow::anyhow;
use std::fmt;

use crate::parse_data_file;

pub struct Day18 { }

impl Default for Day18 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day18 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

fn parse_input(input_string: &String) -> anyhow::Result<Vec<SnailNum>> {
    let mut output = Vec::new();
    for line in input_string.lines() {
        let (num, remaining) = parse_line(line.trim())?;
        if remaining.len() != 0 {
            return Err(anyhow!("Failed to fully parse line! Num: {}, remaining: {}, line: {}", num, remaining, line));
        }

        output.push(num);
    }

    Ok(output)
}

fn parse_line(line: &str) -> anyhow::Result<(SnailNum, &str)> {
    let (front, remaining) = line.split_at(1);
    match front {
        "[" => {
            let (left, left_remaining) = parse_line(remaining)?;
            let (left_front, right_str) = left_remaining.split_at(1);
            if left_front != "," {
                return Err(anyhow!("Parsed left ({}) , but no comma separating it from the right of the number! {}", left, left_remaining));
            }
            let (right, right_remaining) = parse_line(right_str)?;
            let (right_front, final_str) = right_remaining.split_at(1);
            if right_front != "]" {
                return Err(anyhow!("Parsed right ({}) , but no closing bracket found! {}", right, right_remaining));
            }
            Ok((SnailNum::from((left, right)), final_str))
        },
        // 0-9
        s if s.parse::<u8>().is_ok() => Ok((SnailNum::from(s.parse::<u8>().unwrap()), remaining)),
        _ => Err(anyhow!("Unexpected character! {}", front))
    }
}

#[derive(Debug, Clone)]
struct SnailNum {
    val: Option<u8>,
    left: Option<Rc<RefCell<SnailNum>>>,
    right: Option<Rc<RefCell<SnailNum>>>
}

impl SnailNum {
    fn is_val(&self) -> bool {
        self.val.is_some()
    }
}

impl From<u8> for SnailNum {
    fn from(value: u8) -> Self {
        Self { val: Some(value), left: None, right: None }
    }
}

impl From<(SnailNum, SnailNum)> for SnailNum {
    fn from(tuple: (SnailNum, SnailNum)) -> Self {
        Self { 
            val: None, 
            left: Some(Rc::new(RefCell::new(tuple.0))),
            right: Some(Rc::new(RefCell::new(tuple.1)))
        }
    }
}

impl fmt::Display for SnailNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.val, self.left.as_ref(), self.right.as_ref()) {
            (Some(val), None, None) => write!(f, "{}", val),
            (None, Some(left), Some(right)) => write!(f, "[{}, {}]", RefCell::borrow(&left), RefCell::borrow(&right)),
            _ => Err(fmt::Error::default())
        }
    }
}

fn main() -> anyhow::Result<()> {
    let data = parse_data_file(super::YEAR, 18)?;
    let input: Vec<Rc<RefCell<SnailNum>>> = parse_input(&data)?.into_iter().map(|snail_num| Rc::new(RefCell::new(snail_num))).collect();

    let mut input_iter = input.iter();
    let mut output = input_iter.next().expect("Expected the input to have at least one number").clone();
    while let Some(next) = input_iter.next() {
        output = Rc::new(RefCell::new(SnailNum {
            left: Some(output),
            right: Some(next.clone()),
            val: None
        }));
        reduce_number(output.clone())?;
    }

    // Puzzle 1
    println!("Output number: {}", output.as_ref().borrow());
    println!("Output magnitude: {}", get_magnitude(output));

    // Puzzle 2
    let mut max_magnitude = 0;
    for i in 0..input.len() {
        for j in i+1..input.len() {
            // Try both directions.
            let magnitude = single_sum(&data, i, j)?;
            if magnitude > max_magnitude { max_magnitude = magnitude }
            let magnitude = single_sum(&data, j, i)?;
            if magnitude > max_magnitude { max_magnitude = magnitude }
        }
    }
    println!("Output single sum magnitude: {}", max_magnitude);

    Ok(())
}

fn single_sum(input_string: &String, left: usize, right: usize) -> anyhow::Result<u32> {
    // Re-parsing the input so it's unmodified. There *should* be a way to do this in rust, but
    // been working on this for too long to try and figure that out :/
    let input: Vec<Rc<RefCell<SnailNum>>> = parse_input(input_string)?.into_iter().map(|snail_num| Rc::new(RefCell::new(snail_num))).collect();

    let number = Rc::new(RefCell::new(SnailNum {
        val: None,
        left: Some(input[left].clone()),
        right: Some(input[right].clone())
    }));
    reduce_number(number.clone())?;
    Ok(get_magnitude(number))
}

fn reduce_number(number: Rc<RefCell<SnailNum>>) -> anyhow::Result<()> {
    let (mut explosion, mut split) = (true, true);
    while explosion || split {
        (explosion, split) = (false, false);
        // Handle explositions first
        let (_, _, explosion_output) = explode_number(number.clone(), 0)?;
        if explosion_output {
            explosion = true;
            continue;
        }
        split = split_number(number.clone());
    }

    Ok(())
}

fn explode_number(number: Rc<RefCell<SnailNum>>, depth: u8) -> anyhow::Result<(Option<u8>, Option<u8>, bool)> {
    // base case
    let mut number_ref = RefCell::borrow_mut(&number);
    if number_ref.is_val() {
        return Ok((None, None, false));
    }

    if depth > 3 {
        // get references to the left and right values.
        let left_val = number_ref.left.clone().ok_or(anyhow!("Non-leaf node should have a left! {}", number_ref))?;
        let right_val = number_ref.right.clone().ok_or(anyhow!("Non-leaf node should have a right! {}", number_ref))?;
        let left_num = left_val.as_ref().borrow().val.ok_or(anyhow!("Expecting left {:?} to have a value", left_val))?;
        let right_num = right_val.as_ref().borrow().val.ok_or(anyhow!("Expecting right {:?} to have a value", right_val))?;

        // remove the left and right values from the number and update the value to zero.
        number_ref.left = None;
        number_ref.right = None;
        number_ref.val = Some(0);

        // Return the left and right values.
        return Ok((Some(left_num), Some(right_num), true));
    }

    let (left_output, right_val, left_explode) = explode_number(number_ref.left.clone().expect("expect non-leaf number to have a left!"), depth + 1)?;
    // If left has exploded, handle it.
    if left_explode {
        if let Some(right_num) = right_val {
            let mut current = number_ref.right.clone().expect("expect non-leaf number to have a right!");
            while current.as_ref().borrow().left.is_some() {
                current = current.clone().as_ref().borrow().left.clone().unwrap();
            }
            let mut current_num = current.as_ref().borrow_mut();
            current_num.val = Some(current_num.val.expect("Expecting leaf when finding the right most value") + right_num);
        }
        Ok((left_output, None, left_explode))
    } 
    // Else, try and explode the right
    else {
        let (left_val, right_output, right_explode) = explode_number(number_ref.right.clone().expect("expect non-leaf number to have a left!"), depth + 1)?;
        if let Some(left_num) = left_val {
            let mut current = number_ref.left.clone().expect("expect non-leaf number to have a left!");
            while current.as_ref().borrow().right.is_some() {
                current = current.clone().as_ref().borrow().right.clone().unwrap();
            }
            let mut current_num = current.as_ref().borrow_mut();
            current_num.val = Some(current_num.val.expect("Expecting leaf when finding the right most value") + left_num);
        }
        Ok((None, right_output, right_explode))
    }
}

fn split_number(number: Rc<RefCell<SnailNum>>) -> bool {
    let mut number_ref = RefCell::borrow_mut(&number);
    if let Some(val) = number_ref.val {
        if val > 9 {
            number_ref.left = Some(Rc::new(RefCell::new(SnailNum::from(val / 2))));
            number_ref.right = Some(Rc::new(RefCell::new(SnailNum::from((val + 1)/ 2))));
            number_ref.val = None;
            true
        } else {
            false
        }
    } else {
        let left_split = split_number(number_ref.left.clone().expect("Expect non-leaf to have left in split!"));
        if left_split {
            true
        } else {
            split_number(number_ref.right.clone().expect("Expect non-leaf to have left in split!"))
        }
    }
}

fn get_magnitude(number: Rc<RefCell<SnailNum>>) -> u32 {
    let number_ref = RefCell::borrow(&number);
    let left = number_ref.left.clone().unwrap();
    let left_val = left.as_ref().borrow().val.map(u32::from).unwrap_or_else(|| get_magnitude(left.clone())); 
    let right = number_ref.right.clone().unwrap();
    let right_val = right.as_ref().borrow().val.map(u32::from).unwrap_or_else(|| get_magnitude(right.clone())); 
    3 * left_val + 2 * right_val
}
