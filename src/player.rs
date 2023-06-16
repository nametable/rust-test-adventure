use std::rc::Rc;

use crate::{item::{Item, Action}, room::Room};


#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub inventory_capacity: u16,
    pub inventory: Vec<Item>,
    pub current_room: Rc<Room>,
    pub actions: Vec<Box<dyn Action>>,
}