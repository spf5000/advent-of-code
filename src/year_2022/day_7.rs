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
    parent: Option<Rc<RefCell<Directory>>>,
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

    fn cd(&self, dir: &str) -> anyhow::Result<Rc<RefCell<Directory>>> {
        if dir == ".." {
            return if let Some(parent) = self.parent.clone() {
                Ok(parent)
            } else {
                Err(anyhow!("Trying to go up a directory in a directory with no parent! {:#?}", self))
            }
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

    fn get_size(&self) -> anyhow::Result<u32> {
        let mut output = 0;
        for file in self.children.values() {
            output += match file {
                File::Directory(dir) => dir.borrow().get_size()?,
                File::File(file) => file.size
            }
        }
        Ok(output)
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
            Self::Directory(d) => d.borrow().name.clone(),
            Self::File(f) => f.name.clone()
        }
    }

    fn get_size(&self) -> anyhow::Result<u32> {
        match &self {
            File::File(file) => Ok(file.size),
            File::Directory(dir) => dir.borrow().get_size()
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
    let root = traverse_fs(input)?;
    let dirs = traverse_for_dirs(&File::Directory(root))?;
    // NOTE: There is definitely more clever ways of solving this. Should probably capture the
    // childer in a flatter manner and just get the size as we traverse instead of traversing the
    // FS after creating it with the initial traversal.
    let mut output = 0;
    for dir in dirs {
        let dir_size = dir.borrow().get_size()?;
        if dir_size <= 100000 {
            output += dir_size
        }
    }

    println!("Part 1 Answer is {}", output);

    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let input = parse_data_file(super::YEAR, 7)?;
    let root = traverse_fs(input)?;
    let space_needed = 30000000 - (70000000 - root.borrow().get_size()?);
    let dirs = traverse_for_dirs(&File::Directory(root))?;

    // get the amount of space needed to be freed.
    let mut output = 70000000;

    for dir in dirs {
        let dir_size = dir.borrow().get_size()?;
        if dir_size >= space_needed && dir_size < output {
            println!("Updating the deleted dir to {}", dir.borrow().name);
            output = dir_size
        }
    }

    println!("Part 2 Answer is {}", output);

    Ok(())
}

fn parse_input(input: String) -> anyhow::Result<Vec<Line>> {
    input.lines().map(Line::from_str).collect()
}

fn traverse_for_dirs(current: &File) -> anyhow::Result<Vec<Rc<RefCell<Directory>>>> {
    let output = match current {
        File::File(_) => Vec::new(),
        File::Directory(dir) => {
            let mut output = vec![dir.clone()];
            for file in dir.borrow().children.values() {
                output.extend(traverse_for_dirs(file)?);
            }
            output
        }
    };

    Ok(output)
}

fn traverse_fs(input: String) -> anyhow::Result<Rc<RefCell<Directory>>> {
    let input = parse_input(input)?;
    let root = Rc::new(RefCell::new(Directory::new("/".to_string(), None)));
    let mut current = root.clone();

    let mut lines = input.into_iter().peekable();

    // ensure the first line is cd /
    match lines.next() {
        Some(Line::Command(Command::CD { dir })) if dir == "/".to_string() => (),
        _ => bail!("First line wasn't \"cd /\"!")
    }
    
    while let Some(line) = lines.next() {
        match line {
            Line::Command(Command::CD { dir }) => {
                let new_dir = current.borrow().cd(&dir)?;
                current = new_dir;
            },
            Line::Command(Command::LS) => {
                while let Some(Line::Output(_)) = lines.peek() {
                    if let Some(Line::Output(file_deets)) = lines.next() {
                        if file_deets.trim().starts_with("dir ") {
                            let mut split = file_deets.trim().split_whitespace();
                            split.next().unwrap();
                            let child_name = split.next().ok_or(anyhow!("Directory ls output didn't include a directory name! {}", file_deets))?;
                            let child = Directory::new(child_name.to_string(), Some(current.clone()));
                            current.borrow_mut().add_file(File::Directory(Rc::new(RefCell::new(child))))?;
                        } else {
                            current.borrow_mut().add_file(File::File(PlainFile::from_str(&file_deets)?))?;
                        }
                    } else {
                        unreachable!("Verified with the peek beforehand!")
                    }
                }
            },
            Line::Output(_) => bail!("Found an output line when it should have been handled in the LS command arm! {:#?}", line)
        }
    }

    Ok(root)
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
