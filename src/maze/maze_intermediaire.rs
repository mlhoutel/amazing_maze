// #![feature(derive_default_enum)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum Exploration {
    UnExplored, // Left remaining | Right remaining
    PaExplored, // Left explored  | Right remaining
    Explored,   // Left explored  | Right explored
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

    fn explore_trace(self: &Maze, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                trace.push(label.to_string());

                if *status.borrow() == Exploration::UnExplored {
                    status.replace(Exploration::PaExplored);
                    left.explore_trace(trace);
                } else if *status.borrow() == Exploration::PaExplored {
                    status.replace(Exploration::Explored);
                    right.explore_trace(trace);
                }
            }
            Maze::Leaf { label } => {
                trace.push(label.to_string());
            }
        }
    }
}