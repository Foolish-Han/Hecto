
use crate::prelude::*;

use std::{convert::TryFrom, usize};

use crossterm::event::Event;

mod edit;
mod movecommand;
mod system;

pub use edit::Edit;
pub use movecommand::Move;
pub use system::System;

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {:?}", key_event)),
            Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            }))),
            _ => Err(format!("Event not supported: {:?}", value)),
        }
    }
}
