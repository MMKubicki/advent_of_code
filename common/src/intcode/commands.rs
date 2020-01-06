use crate::intcode::Computer;

use std::error;
use std::fmt;

pub struct Change {
    pub position: usize,
    pub value: u32,
}

pub trait Command {
    fn get_intcode(&self) -> usize;
    fn apply(&self, computer: &Computer) -> Result<Vec<Change>, CommandError>;
}

pub trait SimpleCommand {
    fn get_intcode(&self) -> usize;
    fn get_command_name(&self) -> &'static str;
    fn simple_apply(&self, param_one: u32, param_two: u32) -> Result<u32, CommandError>;
}

impl<T> Command for T
where
    T: SimpleCommand,
{
    fn get_intcode(&self) -> usize {
        T::get_intcode(self)
    }

    fn apply(&self, computer: &Computer) -> Result<Vec<Change>, CommandError> {
        let ptr = match computer.get_pointer() {
            None => return Err(CommandError::ApplyError(T::get_command_name(self))),
            Some(value) => value,
        };

        let ptr1 = computer.get_value(ptr + 1) as usize;
        let ptr2 = computer.get_value(ptr + 2) as usize;

        let val1 = computer.get_value(ptr1);
        let val2 = computer.get_value(ptr2);

        let result = match T::simple_apply(self, val1, val2) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        Ok(vec![Change {
            position: computer.get_value(ptr + 3) as usize,
            value: result,
        }])
    }
}

#[derive(fmt::Debug, Copy, Clone, Hash)]
pub enum CommandError {
    ApplyError(&'static str),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            CommandError::ApplyError(i) => write!(f, "Error applying Command: {}", i),
        }
    }
}
impl error::Error for CommandError {}

pub mod default {
    use crate::intcode::commands::{Command, CommandError, SimpleCommand};

    pub fn get_commands() -> Vec<Box<dyn Command>> {
        vec![Box::new(Add::default()), Box::new(Multiply::default())]
    }

    #[derive(Default)]
    struct Add;
    #[derive(Default)]
    struct Multiply;

    impl SimpleCommand for Add {
        fn get_intcode(&self) -> usize {
            1
        }

        fn get_command_name(&self) -> &'static str {
            "Add"
        }

        fn simple_apply(&self, param_one: u32, param_two: u32) -> Result<u32, CommandError> {
            Ok(param_one + param_two)
        }
    }

    impl SimpleCommand for Multiply {
        fn get_intcode(&self) -> usize {
            2
        }

        fn get_command_name(&self) -> &'static str {
            "Multiply"
        }

        fn simple_apply(&self, param_one: u32, param_two: u32) -> Result<u32, CommandError> {
            Ok(param_one * param_two)
        }
    }
}
