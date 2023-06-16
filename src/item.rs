use std::rc::Rc;
use core::fmt::Debug;


#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub actions: Vec<Box<dyn Action>>,
    pub visible: bool
}

pub trait Action {
    fn execute(&self);
}

impl Item {}

impl Debug for dyn Action {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Stuff {}", 444)
    }
}