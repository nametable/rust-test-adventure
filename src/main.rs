use repl_rs::{Command, Parameter, Result, Value};
use repl_rs::{Convert, Repl};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::item::Item;
use crate::player::Player;
use crate::room::{Room, RoomConnector};

mod item;
mod player;
mod room;

#[derive(Default)]
struct Context {
    list: VecDeque<String>,
}

// Append name to list
fn append(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    let name: String = args["name"].convert()?;
    context.list.push_back(name);
    let list: Vec<String> = context.list.clone().into();

    Ok(Some(list.join(", ")))
}

// Prepend name to list
fn prepend(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    let name: String = args["name"].convert()?;
    context.list.push_front(name);
    let list: Vec<String> = context.list.clone().into();

    Ok(Some(list.join(", ")))
}

fn main() -> Result<()> {
    
    let grey_room = Room {
        name: "Room X".into(),
        description:
        "the grey colorless surroundings keep you in a daze. there is stuff around you"
        .into(),
        items: vec![],
        rooms: vec![],
    };
    let grey_room_ref = Rc::new(grey_room);

    let test_room = room::Room {
        name: "Start room".into(),
        description: "Testing".into(),
        items: vec![Item {
            name: "wrench".into(),
            description: "a rusty wrench, seem to be 7/16 inch".into(),
            actions: vec![],
            visible: true,
        }],
        rooms: vec![RoomConnector {
            name: "blue door 1".into(),
            description: "a simple blue door... might lead to something".into(),
            room: grey_room_ref.clone(),
        },
        RoomConnector {
            name: "red door 1".into(),
            description: "a simple red door... might lead to something".into(),
            room: grey_room_ref.clone(),
        }],
    };

    let test_room_ref = Rc::new(test_room);

    let player = Player {
        name: "Joe".into(),
        inventory_capacity: 12,
        inventory: vec![],
        current_room: test_room_ref.clone(),
        actions: vec![],
    };
    
    println!("{:#?}", player);
    
    // let mut repl = Repl::new(Context::default())
    //     .with_name("MyApp")
    //     .with_version("v0.1.0")
    //     .with_description("My very cool app")
    //     .use_completion(true)
    //     .add_command(
    //         Command::new("append", append)
    //             .with_parameter(Parameter::new("name").set_required(true)?)?
    //             .with_help("Append name to end of list"),
    //     )
    //     .add_command(
    //         Command::new("prepend", prepend)
    //             .with_parameter(Parameter::new("name").set_required(true)?)?
    //             .with_help("Prepend name to front of list"),
    //     );
    // repl.run()

    Result::Ok(())
}
