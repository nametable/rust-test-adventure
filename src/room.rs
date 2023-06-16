
use std::rc::Rc;

use crate::item::Item;

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub items: Vec<Item>,
    pub rooms: Vec<RoomConnector>
}

#[derive(Debug)]
pub struct RoomConnector {
    pub name: String,
    pub description: String,
    pub room: Rc<Room>
}