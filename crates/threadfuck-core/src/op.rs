//! Instruction set for the Threadfuck language.
//!
//! Purpose
//! -------
//! Defines the core operation set for Threadfuck and provides conversion
//! from source characters into typed operations.
//!
//! Design
//! ------
//! Threadfuck preserves Brainfuck's core operations and extends them with
//! a minimal concurrency-oriented instruction set. Each valid source
//! character maps to exactly one ``Op`` variant.

use std::thread::Thread;

use crate::error::ThreadfuckError;

/// A single parsed threadfuck instruction.
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum Op { 
    /// Move the current task's data pointer one cell to the right.
    MoveRight,
    
    /// Move the current task's data pointer one cell to the left. 
    MoveLeft,
    
    /// Increment the value in the current tape cell. 
    Increment,
    
    /// Decrement the value in the current tape cell.
    Decrement,

    /// Write the current tape cell to the output stream.
    Output,

    /// Read one byte into the current tape cell.
    Input,

    /// Begin a conditional loop.
    LoopStart,

    /// End a conditional loop.
    LoopEnd,

    /// Spawn a new task from the current execution context.
    ForkTask,

    /// Yield execution voluntarily.
    Yield,

    /// Halt the current task.
    Halt,
}

impl Op {
    /// Return the source character for this opcode.
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::MoveLeft => '>',
            Self::MoveRight => '<',
            Self::Increment => '+',
            Self::Decrement => '-',
            Self::Output => '.',
            Self::Input => ',',
            Self::LoopStart => '[',
            Self::LoopEnd => ']',
            Self::ForkTask => 't',
            Self::Yield => 'y',
            Self::Halt => '!',
        }
    }
}

impl TryFrom<char> for Op{
    type Error = ThreadfuckError;
    
    fn try_from(value: char) -> Result<Self,Self::Error> {
        match value { 
            '>' => Ok(Self::MoveRight),
            '<' => Ok(Self::MoveLeft),
            '+' => Ok(Self::Increment),
            '-' => Ok(Self::Decrement),
            '.' => Ok(Self::Output),
            ',' => Ok(Self::Input),
            '[' => Ok(Self::LoopStart),
            ']' => Ok(Self::LoopEnd),
            't' => Ok(Self::ForkTask),
            'y' => Ok(Self::Yield),
            '!' => Ok(Self::Halt),
            other => Err(ThreadfuckError::InvalidOpcode((other)))

        }
    }
}
