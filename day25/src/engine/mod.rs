use regex::Regex;

use intcode;

use crate::PROGRAM;

lazy_static! {
    static ref COMMAND_RE: Regex = Regex::new(r###"(?:\s*\n)*(?P<description>.+\n)\nCommand\?\n"###).expect("invalid command re");
    static ref ROOM_RE: Regex = Regex::new(r###"(?:\s*\n)*== (?P<room_name>.+) ==\n(?P<room_description>.+\n)(?:\n(?P<doors>Doors here lead:\n(?:- (?:(?:east)|(?:west)|(?:north)|(?:south))\n)+))?(?:\n(?P<items>Items here:\n(?:- [^\n]+)\n)+)?\nCommand\?\n"###).expect("invalid room re");
    static ref TAKE_RE: Regex = Regex::new(r###"(?:\s*\n)*You take the (?P<item>.+)\.\n\nCommand\?\n"###).expect("invalid take re");
    static ref TAKE_INVALID_RE: Regex = Regex::new(r###"(?:\s*\n)*You don't see that item here\.\n\nCommand\?\n"###).expect("invalid take re");
    static ref DROP_RE: Regex = Regex::new(r###"(?:\s*\n)*You drop the (?P<item>.+)\.\n\nCommand\?\n"###).expect("invalid take re");
    static ref DROP_INVALID_RE: Regex = Regex::new(r###"(?:\s*\n)*You don't have that item\.\n\nCommand\?\n"###).expect("invalid take re");    
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub enum Command {
    Inventory,
    Move(Direction),
    Take(Item),
    Drop(Item),
}

pub enum LoadCheckpointError {
    CheckpointNotFound,
}

pub struct Room(String);

pub struct Item(String);

pub struct Engine {
    cpu: intcode::CPU,
}

impl Engine {
    pub fn new() -> Self {
	Self {
	    cpu: intcode::CPU::new(PROGRAM.to_vec(), 0, None),
	}
    }

    pub fn save_checkpoint(&mut self, checkpoint_name: String) {
	todo!()
    }

    pub fn load_checkpoint(&mut self, checkpoint_name: String) -> Result<(), LoadCheckpointError> {
	todo!()
    }

    pub fn list_checkpoints(&self) -> Vec<String> {
	todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door_re() {
	let room = r"


== Hull Breach ==
You got in through a hole in the floor here. To keep your ship from also freezing, the hole has been sealed.

Doors here lead:
- east
- south
- west

Command?
";
	
	let captures = ROOM_RE.captures(room).expect("room captures");

	assert_eq!(captures.name("room_name").expect("room_name").as_str(), "Hull Breach");
	assert_eq!(captures.name("room_description").expect("room_description").as_str(), "You got in through a hole in the floor here. To keep your ship from also freezing, the hole has been sealed.\n");
	assert_eq!(captures.name("doors").expect("doors").as_str(), r"Doors here lead:
- east
- south
- west
");
    }

    #[test]
    fn test_door_items_re() {
	let room = r"


== Hot Chocolate Fountain ==
Somehow, it's still working.

Doors here lead:
- north

Items here:
- photons

Command?
";
	
	let captures = ROOM_RE.captures(room).expect("room captures");

	assert_eq!(captures.name("room_name").expect("room_name").as_str(), "Hot Chocolate Fountain");
	assert_eq!(captures.name("room_description").expect("room_description").as_str(), "Somehow, it's still working.\n");
	assert_eq!(captures.name("doors").expect("doors").as_str(), r"Doors here lead:
- north
");
	assert_eq!(captures.name("items").expect("items").as_str(), r"Items here:
- photons
");
    }

    #[test]
    fn test_take_re() {
	let take = r"
You take the mug.

Command?
";

	let captures = TAKE_RE.captures(take).expect("take captures");

	assert_eq!(captures.name("item").expect("item").as_str(), "mug");
    }

    #[test]
    fn test_take_invalid_re() {
	let take_invalid = r"

You don't see that item here.

Command?
";

	assert!(TAKE_INVALID_RE.is_match(take_invalid));
    }

    #[test]
    fn test_drop_re() {
	let drop = r"
You drop the mug.

Command?
";

	let captures = DROP_RE.captures(drop).expect("drop captures");

	assert_eq!(captures.name("item").expect("item").as_str(), "mug");
    }

    #[test]
    fn test_drop_invalid_re() {
	let drop_invalid = r"

You don't have that item.

Command?
";

	assert!(DROP_INVALID_RE.is_match(drop_invalid));
    }

    #[test]
    fn test_command_re() {
	let command = r"

You don't have that item.

Command?
";

	let captures = COMMAND_RE.captures(command).expect("command");
	
	assert_eq!(captures.name("description").expect("description").as_str(), "You don't have that item.\n");
    }
}
