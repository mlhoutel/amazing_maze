#[path = "maze/maze.rs"] mod maze;

use std::cell::RefCell;
use std::rc::Rc;
use crate::maze::Exploration;
use crate::maze::Maze::{Branch, Leaf};

fn main() {
    let leaf2 = Rc::new(Leaf{label: String::from("2")});
    let leaf4 = Rc::new(Leaf{label: String::from("4")});
    let leaf5 = Rc::new(Leaf{label: String::from("5")});
    let leaf8 = Rc::new(Leaf{label: String::from("8")});
    let branch3 = Rc::new(Branch{label: String::from("3"), left: Rc::clone(&leaf4), right: Rc::clone(&leaf5), status: RefCell::from(Exploration::UnExplored) });
    let branch1 = Rc::new(Branch{label: String::from("1"), left: Rc::clone(&leaf2), right: Rc::clone(&branch3), status: RefCell::from(Exploration::UnExplored) });
    let branch7 = Rc::new(Branch{label: String::from("7"), left: Rc::clone(&leaf5), right: Rc::clone(&leaf8), status: RefCell::from(Exploration::UnExplored) });
    let branch6 = Rc::new(Branch{label: String::from("6"), left: Rc::clone(&branch3), right: Rc::clone(&branch7), status: RefCell::from(Exploration::UnExplored) });
    let branch0 = Rc::new(Branch{label: String::from("0"), left: Rc::clone(&branch1), right: Rc::clone(&branch6), status: RefCell::from(Exploration::UnExplored) });

    //println!("{:?}", branch0.explore());
    println!("{:?}", branch0.explore_multi());
}
