use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use anyhow::anyhow;

use crate::parse_data_file;

pub struct Day12 { }

impl Default for Day12 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day12 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

const START: &'static str = "start";
const END: &'static str = "end";

fn parse_graph<'a>(input_string: &'a str) -> anyhow::Result<Rc<RefCell<Node<'a>>>> {
    let mut output = HashMap::new();
    for line in input_string.lines() {
        let mut split = line.split('-');
        let left = split.next().expect(&format!("Line should have two values split on '-': {}", line));
        let right = split.next().expect(&format!("Line should have two values split on '-': {}", line));
        let left_node: Rc<RefCell<Node<'a>>> = output.remove(left).unwrap_or(Rc::new(RefCell::new(Node::new(left))));
        let right_node: Rc<RefCell<Node<'a>>> = output.remove(right).unwrap_or(Rc::new(RefCell::new(Node::new(right))));
        left_node.borrow_mut().edges.push(right_node.clone());
        right_node.borrow_mut().edges.push(left_node.clone());
        output.insert(left, left_node.clone());
        output.insert(right, right_node.clone());
    }

    output.remove("start").ok_or(anyhow!("Never parsed a start node!"))
}

fn main() -> anyhow::Result<()> {
    // let input_string = parse_data_file("test.txt")?;
    let data = parse_data_file(super::YEAR, 12)?;
    let graph = parse_graph(&data)?;

    let mut visited = HashSet::new();
    let answer = search(graph, &mut visited, true);

    println!("The answer is {}", answer);

    Ok(())
}

fn search<'a>(node: Rc<RefCell<Node<'a>>>, visited: &mut HashSet<&'a str>, can_revisit: bool) -> u32 {
    let node_val = node.borrow().val;

    // end, return
    if node_val == END {
        return 1;
    }

    // lowercase
    let is_lowercase = node_val.find(char::is_uppercase).is_none();
    let mut revisit = false;
    match (is_lowercase, visited.contains(node_val), node_val == START, can_revisit) {
        (false, _, _, _) => (), // Ignore non lowercase str
        (true, false, _, _) => { visited.insert(node_val); } // first time on node
        (true, true, true, _) | (true, true, false, false) => return 0, // can't revisit START or already used revisit
        (true, true, false, true) => revisit = true, // using revisit
    };

    let mut output = 0;
    for edge in node.borrow().edges.iter() {
        output += search(edge.clone(), visited, can_revisit && !revisit);
    }

    // remove this node from visited at the end if this isn't a revisit
    if is_lowercase && !revisit {
        visited.remove(node_val);
    }

    output
}

#[derive(Debug)]
struct Node<'a> {
    val: &'a str,
    edges: Vec<Rc<RefCell<Node<'a>>>>
}

impl <'a> Node<'a> {
    fn new(val: &'a str) -> Self {
        Self { val, edges: Vec::new() }
    }
}
