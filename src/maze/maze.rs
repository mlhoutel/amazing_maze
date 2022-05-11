// #![feature(derive_default_enum)]

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
// #[derive(Default)]
pub enum Exploration {
    // #[default]
    UnExplored,
    Explored,
    PartiallyExplored,
}

pub enum Maze {
    Branch { label: String, left: Rc<Maze>, right: Rc<Maze>, status: RefCell<Exploration> },
    Leaf { label: String },
}

impl Maze {
    pub fn explore(self: &Maze) -> Vec<String> {
        let mut trace = Vec::new();
        self.explore_trace(&mut trace);
        return trace.clone();
    }

    pub fn explore_multi(self: Rc<Maze>) -> Vec<String> {
        let mut work = vec![Rc::clone(&self)];
        let mut trace = vec![];
        while work.len() != 0 {
            let node = work.pop().expect("unexpected");
            node.explore_stack(&mut work, &mut trace);
            println!("trace so far: {:?}", trace);
        }
        return trace.clone();
    }
    fn explore_stack(self: Rc<Maze>, work: &mut Vec<Rc<Maze>>, trace: &mut Vec<String>) {
        let node = self.borrow();
        match node {
            Maze::Branch { label, left, right, status } => {
                trace.push(label.to_string());

                if *status.borrow() == Exploration::UnExplored {
                    work.push(left.clone());
                    status.replace(Exploration::PartiallyExplored);
                }
                if *status.borrow() == Exploration::PartiallyExplored {
                    work.push(right.clone());
                    status.replace(Exploration::Explored);
                }
            }
            Maze::Leaf { label } => {
                trace.push(label.to_string());
            }
        }
    }

    fn explore_trace(self: &Maze, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                trace.push(label.to_string());

                if *status.borrow() == Exploration::UnExplored {
                    status.replace(Exploration::Explored);
                    left.explore_trace(trace);
                    right.explore_trace(trace);
                }
            }
            Maze::Leaf { label } => {
                trace.push(label.to_string());
            }
        }
    }
}