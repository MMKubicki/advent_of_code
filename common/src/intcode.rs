pub mod commands;

use crate::intcode::commands::Command;
use std::collections::HashMap;
use std::convert;
use std::error;
use std::fmt::{self, Debug};
use std::hash::Hash;

pub struct Computer {
    /// Stores command-input and memory of computer
    program: Vec<u32>,
    /// Stores pointer to next command
    /// If none -> Computer completed a 99 command
    pointer: Option<usize>,
    ///Commands for Computer
    commands: HashMap<usize, Box<dyn Command>>,
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum ComputerStatus {
    Running,
    Complete,
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum SetProgramError {
    NoEndCommandError,
    UnreachableEndCommandError,
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum SetValueError {
    InvalidPositionError(usize),
}

pub struct Iter<'a> {
    computer: &'a mut Computer,
}

impl Computer {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        for cmd in self::commands::default::get_commands() {
            commands.insert(cmd.get_intcode(), cmd);
        }

        Computer {
            program: vec![99],
            pointer: None,
            commands,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.pointer.is_none()
    }

    pub fn is_running(&self) -> bool {
        self.pointer.is_some()
    }

    pub fn get_value(&self, position: usize) -> u32 {
        self.program[position]
    }

    pub fn get_pointer(&self) -> Option<usize> {
        self.pointer
    }

    pub fn set_program(&mut self, program: Vec<u32>) -> Result<(), SetProgramError> {
        self.program = program;
        self.pointer = Some(0);

        Ok(())

        //TODO: Think more about validating...
        //end code could be calculated at runtime...

        /*
        if let Some(pos) = program.iter().position(|v| v == &99) {
            if pos % 4 == 0 {
                self.program = program;
                self.pointer = Some(0);
                Ok(())
            } else {
                Err(SetProgramError::UnreachableEndCommandError)
            }
        } else {
            Err(SetProgramError::NoEndCommandError)
        }
        */
    }

    pub fn set_value(&mut self, position: usize, value: u32) -> Result<(), SetValueError> {
        if position >= self.program.len() {
            Err(SetValueError::InvalidPositionError(position))
        } else {
            self.program[position] = value;
            Ok(())
        }
    }

    pub fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.get_intcode(), cmd);
    }

    pub fn clone_memory(&self) -> Vec<u32> {
        self.program.clone()
    }

    pub fn step(&mut self) -> ComputerStatus {
        //check if computer can still run
        let ptr = match self.pointer {
            None => return ComputerStatus::Complete,
            Some(val) => val,
        };

        let intcode = self.program[ptr] as usize;

        //sanity check if completed previously but not marked as completed before
        if intcode == 99 {
            self.pointer = None;
            return ComputerStatus::Complete;
        }

        //run
        let changes_to_apply = match self.commands[&intcode].apply(self) {
            Ok(value) => value,
            Err(err) => panic!(err),
        };
        for change in changes_to_apply {
            match self.set_value(change.position, change.value) {
                Ok(_) => {}
                Err(err) => panic!(err),
            }
        }

        //increment ptr
        let ptr = ptr + 4;
        return match self.program[ptr] {
            99 => {
                self.pointer = None;
                ComputerStatus::Complete
            }
            _ => {
                self.pointer = Some(ptr);
                ComputerStatus::Running
            }
        };
    }

    pub fn iter(&mut self) -> Iter {
        Iter { computer: self }
    }

    pub fn run_to_end(&mut self) {
        let mut iter = self.iter();
        loop {
            match iter.next() {
                None => break,
                Some(_) => {}
            }
        }
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(ptr) = self.pointer {
            write!(
                f,
                "Computer is running. Current pointer: {}. Next command: {}",
                ptr, self.program[ptr]
            )
        } else {
            write!(f, "Computer is done. Result: {}", self.program[0])
        }
    }
}

impl Default for Computer {
    fn default() -> Self {
        Self::new()
    }
}

impl convert::TryFrom<Vec<u32>> for Computer {
    type Error = SetProgramError;

    fn try_from(value: Vec<u32>) -> Result<Self, Self::Error> {
        let mut computer = Computer::new();
        match computer.set_program(value) {
            Ok(_) => Ok(computer),
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for SetProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            SetProgramError::NoEndCommandError => {
                write!(f, "No end command (99) found in given program.")
            }
            SetProgramError::UnreachableEndCommandError => {
                write!(f, "End command (99) found in unreachable position")
            }
        }
    }
}
impl error::Error for SetProgramError {}

impl fmt::Display for SetValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            SetValueError::InvalidPositionError(ptr) => write!(f, "Position is invalid: {}", ptr),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self.computer.step() {
            ComputerStatus::Running => Some(()),
            ComputerStatus::Complete => None,
        }
    }
}
