use std::borrow::Borrow;
use std::str::FromStr;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use anyhow::{anyhow, bail};

use crate::parse_data_file;

pub struct Day7 { }

impl Default for Day7 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day7 {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

#[derive(Debug, PartialEq)]
enum Line {
    Command(Command),
    Output(String)
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.trim().starts_with("$") {
            Self::Command(Command::from_str(s)?)
        } else {
            Self::Output(s.to_string())
        })
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    CD {dir: String},
    LS
}

impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split_whitespace();
        split.next().ok_or(anyhow!("Failed to parse command from {}", s))?;
        Ok(match split.next() {
            Some("cd") => {
                let dir = split.next().ok_or(anyhow!("Failed to get the directory from the cd command! {}", s))?;
                Self::CD { dir: dir.to_string() }
            },
            Some("ls") => {
                Self::LS
            },
            Some(_) | None => bail!("Cannot parse a command from {}", s)
        })
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell< Directory>>>,
    children: HashMap<String, File>,
}

impl Directory {
    fn new(name: String, parent: Option<Rc<RefCell<Directory>>>) -> Self {
        Self {
            name,
            parent,
            children: HashMap::new()
        }
    }

    fn cd(&mut self, dir: &str) -> anyhow::Result<Rc<RefCell<Directory>>> {
        if dir == ".." {
            println!("Going up!");
            let cloned = self.parent.clone();
            // println!("Cloned: {:#?}", cloned);
            return Ok(cloned.ok_or(anyhow!("Trying to go up a directory in a directory with no parent! {:#?}", self))?)
        }
        match self.children.get(dir) {
            Some(File::Directory(output)) => Ok(output.clone()),
            Some(File::File(_)) => bail!("Trying to change to a non-directory file {}!", dir),
            None => bail!("{} isn't a child of the current directory {}", dir, self.name)
        }
    }

    fn add_file(&mut self, file: File) -> anyhow::Result<()> {
        self.children.insert(file.get_name(), file);
        Ok(())
    }
}

#[derive(Debug)]
enum File {
    Directory(Rc<RefCell<Directory>>),
    File(PlainFile)
}

impl File {
    fn get_name(&self) -> String {
        match self {
            Self::Directory(d) => d.as_ref().borrow().name.clone(),
            Self::File(f) => f.name.clone()
        }
    }
}

#[derive(Debug, PartialEq)]
struct PlainFile {
    name: String,
    size: u32, 
}

impl FromStr for PlainFile {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split_whitespace();
        Ok(Self {
            size: u32::from_str(split.next().ok_or(anyhow!("Failed to parse size out of plain file {}!", s))?)?,
            name: split.next().ok_or(anyhow!("Failed to parse name out of plain file {}!", s))?.to_string()
        })
    }
}

fn part1() -> anyhow::Result<()> {
    let input = parse_data_file(super::YEAR, 7)?;
    traverse_fs(input)?;
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    parse_data_file(super::YEAR, 7)?;
    Ok(())
}

fn parse_input(input: String) -> anyhow::Result<Vec<Line>> {
    input.lines().map(Line::from_str).collect()
}

fn traverse_fs(input: String) -> anyhow::Result<Rc<RefCell<Directory>>> {
    let input = parse_input(input)?;
    let output = Rc::new(RefCell::new(Directory::new("/".to_string(), None)));
    let mut current = output.clone();

    let mut lines = input.into_iter().peekable();

    // ensure the first line is cd /
    match lines.next() {
        Some(Line::Command(Command::CD { dir })) if dir == "/".to_string() => (),
        _ => bail!("First line wasn't \"cd /\"!")
    }
    
    while let Some(line) = lines.next() {
        println!("Acting on line {:#?}", line);
        // println!("Current: {:#?}", current);
        match line {
            Line::Command(Command::CD { dir }) => {
                println!("Changing directory to {}", dir);
                let new_dir = current.borrow_mut().cd(&dir)?;
                println!("updating current");
                current = new_dir;
                println!("current updated");
            },
            Line::Command(Command::LS) => {
                println!("Handling LS");
                while let Some(Line::Output(_)) = lines.peek() {
                    if let Some(Line::Output(file_deets)) = lines.next() {
                        println!("File deetz: {}", file_deets);
                        // directory output;
                        if file_deets.trim().starts_with("dir ") {
                            let mut split = file_deets.trim().split_whitespace();
                            split.next().unwrap();
                            let child_name = split.next().ok_or(anyhow!("Directory ls output didn't include a directory name! {}", file_deets))?;
                            let child = Directory::new(child_name.to_string(), Some(current.clone()));
                            println!("Child! {:#?}", child);
                            current.borrow_mut().add_file(File::Directory(Rc::new(RefCell::new(child))))?;
                        } else {
                            current.borrow_mut().add_file(File::File(PlainFile::from_str(&file_deets)?))?;
                        }

                    } else {
                        unreachable!("Verified with the peek beforehand!")
                    }
                }
                // println!("LS complete! {:#?}", current)
            },
            Line::Output(_) => bail!("Found an output line when it should have been handled in the LS command arm! {:#?}", line)
        }
    }

    Ok(output)
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_test_data_file;

    #[test]
    fn parse_input_test() {
        let input_str = parse_test_data_file(super::super::YEAR, 7).expect("Failed to get test data from file!");
        let input = parse_input(input_str).unwrap();

        let expected_input = vec![
            Line::Command(Command::CD { dir: "/".to_string() }),
            Line::Command(Command::LS),
            Line::Output("dir a".to_string()),
            Line::Output("14848514 b.txt".to_string()),
            Line::Output("8504156 c.dat".to_string()),
            Line::Output("dir d".to_string()),
            Line::Command(Command::CD { dir: "a".to_string() }),
            Line::Command(Command::LS),
            Line::Output("dir e".to_string()),
            Line::Output("29116 f".to_string()),
            Line::Output("2557 g".to_string()),
            Line::Output("62596 h.lst".to_string()),
            Line::Command(Command::CD { dir: "e".to_string() }),
            Line::Command(Command::LS),
            Line::Output("584 i".to_string()),
            Line::Command(Command::CD { dir: "..".to_string() }),
            Line::Command(Command::CD { dir: "..".to_string() }),
            Line::Command(Command::CD { dir: "d".to_string() }),
            Line::Command(Command::LS),
            Line::Output("4060174 j".to_string()),
            Line::Output("8033020 d.log".to_string()),
            Line::Output("5626152 d.ext".to_string()),
            Line::Output("7214296 k".to_string()),
        ];

        assert_eq!(expected_input, input)
    }

    #[test]
    fn part1_test() {
        let input = parse_test_data_file(super::super::YEAR, 7).expect("Failed to get test data from file!");
        let output = traverse_fs(input).unwrap();
    }
}
